"""
User Pattern Learning Service for Predictive Categorization.

This service learns from user's task categorization patterns and provides
intelligent predictions with confidence scores.
"""

import numpy as np
from typing import List, Dict, Tuple, Optional, Any
import logging
from datetime import datetime
from sklearn.ensemble import RandomForestClassifier
from sklearn.preprocessing import LabelEncoder
from sentence_transformers import SentenceTransformer
import pickle
import os
from collections import defaultdict

from app.config import settings

logger = logging.getLogger(__name__)


class UserPatternLearner:
    """
    Learns user's categorization patterns and predicts task categories.

    Uses embeddings + Random Forest for classification with confidence scores.
    Continuously learns from user actions.
    """

    def __init__(self, user_id: str):
        self.user_id = user_id
        self.model_path = os.path.join(
            settings.ML_MODEL_PATH,
            f"user_{user_id}_categorizer.pkl"
        )
        self.encoder_path = os.path.join(
            settings.ML_MODEL_PATH,
            f"user_{user_id}_encoder.pkl"
        )

        # Initialize model and encoder
        self.classifier: Optional[RandomForestClassifier] = None
        self.label_encoder: Optional[LabelEncoder] = None
        self.sentence_model: Optional[SentenceTransformer] = None

        # Training data storage
        self.training_data: List[Dict[str, Any]] = []
        self.min_samples_per_class = 3  # Minimum samples before predictions

        # Load existing model if available
        self._load_model()

    def _get_sentence_model(self) -> SentenceTransformer:
        """Get or initialize sentence transformer."""
        if self.sentence_model is None:
            from app.services.ml import get_model
            self.sentence_model = get_model()
        return self.sentence_model

    def _get_task_text(self, task: Dict[str, Any]) -> str:
        """Extract text from task for embedding."""
        parts = [task.get("title", "")]
        if task.get("body"):
            parts.append(task["body"])
        if task.get("tags"):
            parts.append(" ".join(task["tags"]))
        return " ".join(parts).strip()

    def _get_embedding(self, text: str) -> np.ndarray:
        """Get embedding for text."""
        model = self._get_sentence_model()
        return model.encode([text])[0]

    def _load_model(self):
        """Load existing model and encoder from disk."""
        try:
            if os.path.exists(self.model_path) and os.path.exists(self.encoder_path):
                with open(self.model_path, 'rb') as f:
                    data = pickle.load(f)
                    self.classifier = data['classifier']
                    self.training_data = data['training_data']

                with open(self.encoder_path, 'rb') as f:
                    self.label_encoder = pickle.load(f)

                logger.info(
                    f"Loaded model for user {self.user_id} with "
                    f"{len(self.training_data)} training samples"
                )
        except Exception as e:
            logger.error(f"Error loading model for user {self.user_id}: {e}")
            self.classifier = None
            self.label_encoder = None

    def _save_model(self):
        """Save model and encoder to disk."""
        try:
            os.makedirs(settings.ML_MODEL_PATH, exist_ok=True)

            with open(self.model_path, 'wb') as f:
                pickle.dump({
                    'classifier': self.classifier,
                    'training_data': self.training_data,
                    'updated_at': datetime.utcnow().isoformat()
                }, f)

            with open(self.encoder_path, 'wb') as f:
                pickle.dump(self.label_encoder, f)

            logger.info(f"Saved model for user {self.user_id}")
        except Exception as e:
            logger.error(f"Error saving model for user {self.user_id}: {e}")

    def learn_from_action(self, task: Dict[str, Any], category: str):
        """
        Learn from user's categorization action.

        Args:
            task: The task that was categorized
            category: The category assigned by user (TODO, WAITING, etc.)
        """
        text = self._get_task_text(task)
        embedding = self._get_embedding(text)

        # Add to training data
        self.training_data.append({
            'text': text,
            'embedding': embedding,
            'category': category,
            'timestamp': datetime.utcnow().isoformat()
        })

        # Retrain model if we have enough data
        self._retrain_if_ready()

    def _retrain_if_ready(self):
        """Retrain model if we have enough samples per class."""
        if len(self.training_data) < 5:  # Need at least 5 samples total
            return

        # Count samples per category
        category_counts = defaultdict(int)
        for sample in self.training_data:
            category_counts[sample['category']] += 1

        # Check if we have enough samples per category
        valid_categories = [
            cat for cat, count in category_counts.items()
            if count >= self.min_samples_per_class
        ]

        if len(valid_categories) < 2:  # Need at least 2 categories
            logger.info(
                f"User {self.user_id}: Not enough samples per class for training"
            )
            return

        # Filter training data to only include valid categories
        filtered_data = [
            sample for sample in self.training_data
            if sample['category'] in valid_categories
        ]

        # Prepare training data
        X = np.array([sample['embedding'] for sample in filtered_data])
        y = [sample['category'] for sample in filtered_data]

        # Initialize or update label encoder
        if self.label_encoder is None:
            self.label_encoder = LabelEncoder()

        y_encoded = self.label_encoder.fit_transform(y)

        # Train Random Forest classifier
        self.classifier = RandomForestClassifier(
            n_estimators=100,
            max_depth=10,
            min_samples_split=2,
            random_state=42,
            class_weight='balanced'  # Handle imbalanced classes
        )

        self.classifier.fit(X, y_encoded)

        logger.info(
            f"Trained model for user {self.user_id} with {len(filtered_data)} "
            f"samples across {len(valid_categories)} categories"
        )

        # Save model
        self._save_model()

    def predict_category(
        self,
        task: Dict[str, Any]
    ) -> Optional[Tuple[str, float]]:
        """
        Predict category for a task with confidence score.

        Args:
            task: The task to categorize

        Returns:
            Tuple of (predicted_category, confidence) or None if not ready
        """
        if self.classifier is None or self.label_encoder is None:
            return None

        text = self._get_task_text(task)
        embedding = self._get_embedding(text)

        # Predict probabilities
        probabilities = self.classifier.predict_proba([embedding])[0]

        # Get prediction with highest probability
        predicted_idx = np.argmax(probabilities)
        confidence = probabilities[predicted_idx]
        predicted_category = self.label_encoder.inverse_transform([predicted_idx])[0]

        return predicted_category, float(confidence)

    def batch_categorize(
        self,
        tasks: List[Dict[str, Any]],
        confidence_threshold: float = 0.8
    ) -> Dict[str, List[Dict[str, Any]]]:
        """
        Categorize multiple tasks with confidence filtering.

        Args:
            tasks: List of tasks to categorize
            confidence_threshold: Minimum confidence for auto-categorization

        Returns:
            Dictionary with 'auto' (high confidence) and 'manual' (low confidence)
        """
        result = {
            'auto': [],  # Auto-categorize with high confidence
            'manual': []  # Needs manual confirmation
        }

        if self.classifier is None:
            # No model yet, all need manual categorization
            result['manual'] = [
                {
                    'task': task,
                    'suggestions': []
                }
                for task in tasks
            ]
            return result

        for task in tasks:
            prediction = self.predict_category(task)

            if prediction is None:
                result['manual'].append({
                    'task': task,
                    'suggestions': []
                })
                continue

            category, confidence = prediction

            if confidence >= confidence_threshold:
                # High confidence - auto-categorize
                result['auto'].append({
                    'task': task,
                    'category': category,
                    'confidence': confidence
                })
            else:
                # Low confidence - suggest but require confirmation
                # Get top 3 suggestions
                text = self._get_task_text(task)
                embedding = self._get_embedding(text)
                probabilities = self.classifier.predict_proba([embedding])[0]

                # Get indices of top 3 probabilities
                top_indices = np.argsort(probabilities)[-3:][::-1]

                suggestions = [
                    {
                        'category': self.label_encoder.inverse_transform([idx])[0],
                        'confidence': float(probabilities[idx])
                    }
                    for idx in top_indices
                    if probabilities[idx] > 0.1  # Only suggest if >10% confidence
                ]

                result['manual'].append({
                    'task': task,
                    'suggestions': suggestions
                })

        return result

    def get_statistics(self) -> Dict[str, Any]:
        """Get learning statistics for this user."""
        if not self.training_data:
            return {
                'total_samples': 0,
                'is_trained': False,
                'categories': {}
            }

        category_counts = defaultdict(int)
        for sample in self.training_data:
            category_counts[sample['category']] += 1

        return {
            'total_samples': len(self.training_data),
            'is_trained': self.classifier is not None,
            'categories': dict(category_counts),
            'model_updated_at': self.training_data[-1]['timestamp'] if self.training_data else None
        }


# Global cache of learners per user
_learner_cache: Dict[str, UserPatternLearner] = {}


def get_learner(user_id: str) -> UserPatternLearner:
    """Get or create learner for a user."""
    if user_id not in _learner_cache:
        _learner_cache[user_id] = UserPatternLearner(user_id)
    return _learner_cache[user_id]
