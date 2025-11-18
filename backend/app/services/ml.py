"""
Machine Learning services for JustOrgYou.

Provides AI-powered features like semantic search, tag suggestion,
auto-categorization, and duplicate detection.
"""

from typing import List, Dict, Tuple, Any
import numpy as np
from sentence_transformers import SentenceTransformer
import logging
from sklearn.metrics.pairwise import cosine_similarity

from app.config import settings

logger = logging.getLogger(__name__)

# Global model instance (lazy loaded)
_model: SentenceTransformer = None


def get_model() -> SentenceTransformer:
    """Get or initialize the sentence transformer model."""
    global _model

    if _model is None:
        logger.info(f"Loading model: {settings.SENTENCE_TRANSFORMER_MODEL}")
        _model = SentenceTransformer(settings.SENTENCE_TRANSFORMER_MODEL)
        logger.info("Model loaded successfully")

    return _model


def get_task_text(task: Dict[str, Any]) -> str:
    """Extract searchable text from a task."""
    parts = [task.get("title", "")]

    if task.get("body"):
        parts.append(task["body"])

    if task.get("tags"):
        parts.append(" ".join(task["tags"]))

    return " ".join(parts).strip()


async def semantic_search(
    query: str,
    tasks: List[Dict[str, Any]],
    limit: int = 10,
) -> List[Dict[str, Any]]:
    """
    Perform semantic search on tasks.

    Args:
        query: Search query
        tasks: List of tasks to search
        limit: Maximum number of results to return

    Returns:
        List of tasks sorted by relevance
    """
    if not tasks:
        return []

    model = get_model()

    # Create embeddings
    query_embedding = model.encode([query])[0]
    task_texts = [get_task_text(task) for task in tasks]
    task_embeddings = model.encode(task_texts)

    # Calculate similarities
    similarities = cosine_similarity([query_embedding], task_embeddings)[0]

    # Sort by similarity
    results = []
    for idx, similarity in enumerate(similarities):
        results.append({
            "task": tasks[idx],
            "score": float(similarity),
        })

    # Sort by score and limit
    results.sort(key=lambda x: x["score"], reverse=True)
    return results[:limit]


async def suggest_tags(
    task: Dict[str, Any],
    existing_tags: List[str],
    limit: int = 5,
) -> List[str]:
    """
    Suggest relevant tags for a task.

    Args:
        task: Task to suggest tags for
        existing_tags: List of existing tags in the system
        limit: Maximum number of tags to suggest

    Returns:
        List of suggested tags
    """
    if not existing_tags:
        return []

    model = get_model()

    # Get task text and embedding
    task_text = get_task_text(task)
    task_embedding = model.encode([task_text])[0]

    # Get embeddings for existing tags
    tag_embeddings = model.encode(existing_tags)

    # Calculate similarities
    similarities = cosine_similarity([task_embedding], tag_embeddings)[0]

    # Sort tags by similarity
    tag_scores = list(zip(existing_tags, similarities))
    tag_scores.sort(key=lambda x: x[1], reverse=True)

    # Filter out tags already on the task
    current_tags = set(task.get("tags", []))
    suggested = [
        tag for tag, _ in tag_scores
        if tag not in current_tags
    ]

    return suggested[:limit]


# Keyword classification patterns
KEYWORD_PATTERNS = {
    "INBOX": ["inbox", "capture", "new", "quick"],
    "TODO": ["do", "task", "action", "work", "complete"],
    "WAITING": ["waiting", "blocked", "pending", "hold"],
    "SOMEDAY": ["someday", "maybe", "future", "later"],
    "DONE": ["done", "completed", "finished"],
}


async def auto_categorize(task: Dict[str, Any]) -> Tuple[str, float]:
    """
    Automatically categorize a task.

    Args:
        task: Task to categorize

    Returns:
        Tuple of (suggested_keyword, confidence)
    """
    task_text = get_task_text(task).lower()

    # Check for keyword patterns
    scores = {}
    for keyword, patterns in KEYWORD_PATTERNS.items():
        score = sum(1 for pattern in patterns if pattern in task_text)
        scores[keyword] = score

    # If we found matches, return the best one
    if max(scores.values()) > 0:
        best_keyword = max(scores.items(), key=lambda x: x[1])[0]
        confidence = min(scores[best_keyword] / 3.0, 1.0)
        return best_keyword, confidence

    # Default to TODO with low confidence
    return "TODO", 0.3


async def detect_duplicates(
    tasks: List[Dict[str, Any]],
    threshold: float = 0.85,
) -> List[Dict[str, Any]]:
    """
    Detect duplicate tasks.

    Args:
        tasks: List of tasks to check
        threshold: Similarity threshold for duplicates (0-1)

    Returns:
        List of duplicate groups
    """
    if len(tasks) < 2:
        return []

    model = get_model()

    # Create embeddings
    task_texts = [get_task_text(task) for task in tasks]
    embeddings = model.encode(task_texts)

    # Calculate pairwise similarities
    similarities = cosine_similarity(embeddings)

    # Find duplicates
    duplicates = []
    checked = set()

    for i in range(len(tasks)):
        if i in checked:
            continue

        similar_indices = []
        for j in range(i + 1, len(tasks)):
            if j in checked:
                continue

            if similarities[i][j] >= threshold:
                similar_indices.append(j)
                checked.add(j)

        if similar_indices:
            group = {
                "primary": tasks[i],
                "duplicates": [tasks[j] for j in similar_indices],
                "similarities": [
                    float(similarities[i][j]) for j in similar_indices
                ],
            }
            duplicates.append(group)
            checked.add(i)

    return duplicates


# For testing purposes
async def health_check() -> bool:
    """Check if ML services are healthy."""
    try:
        model = get_model()
        # Test encoding
        _ = model.encode(["test"])
        return True
    except Exception as e:
        logger.error(f"ML health check failed: {e}")
        return False
