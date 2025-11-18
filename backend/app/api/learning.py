from fastapi import APIRouter, HTTPException, Header
from pydantic import BaseModel
from typing import List, Optional, Dict, Any
import logging
import os

from app.services.user_learning import get_learner

logger = logging.getLogger(__name__)
router = APIRouter()


# Request/Response Models
class Task(BaseModel):
    """Task model for API."""
    id: Optional[str] = None
    title: str
    body: Optional[str] = None
    tags: List[str] = []
    keyword: str = "TODO"


class LearnActionRequest(BaseModel):
    """Request to learn from user action."""
    task: Task
    category: str


class PredictCategoryRequest(BaseModel):
    """Request to predict category for a task."""
    task: Task


class PredictCategoryResponse(BaseModel):
    """Response with predicted category."""
    category: Optional[str]
    confidence: Optional[float]
    is_ready: bool


class BatchCategorizeRequest(BaseModel):
    """Request to categorize multiple tasks."""
    tasks: List[Task]
    confidence_threshold: float = 0.8


class CategorySuggestion(BaseModel):
    """Category suggestion with confidence."""
    category: str
    confidence: float


class TaskWithSuggestions(BaseModel):
    """Task with category suggestions."""
    task: Task
    suggestions: List[CategorySuggestion]


class TaskWithCategory(BaseModel):
    """Task with assigned category."""
    task: Task
    category: str
    confidence: float


class BatchCategorizeResponse(BaseModel):
    """Response with categorized tasks."""
    auto: List[TaskWithCategory]
    manual: List[TaskWithSuggestions]


class LearningStatsResponse(BaseModel):
    """Learning statistics for user."""
    total_samples: int
    is_trained: bool
    categories: Dict[str, int]
    model_updated_at: Optional[str]


@router.post("/learn", status_code=201)
async def learn_from_action(
    request: LearnActionRequest,
    user_id: str = Header(..., alias="X-User-Id")
):
    """
    Learn from user's categorization action.

    The system will continuously learn from user's patterns of
    categorizing tasks from inbox.

    Example:
        User moves task "Buy groceries" from INBOX to TODO
        → System learns this pattern for future similar tasks
    """
    try:
        learner = get_learner(user_id)
        learner.learn_from_action(
            task=request.task.model_dump(),
            category=request.category
        )

        stats = learner.get_statistics()

        return {
            "message": "Learning action recorded",
            "stats": stats
        }

    except Exception as e:
        logger.error(f"Learning error for user {user_id}: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/predict", response_model=PredictCategoryResponse)
async def predict_category(
    request: PredictCategoryRequest,
    user_id: str = Header(..., alias="X-User-Id")
):
    """
    Predict category for a single task.

    Returns predicted category with confidence score,
    or None if model is not ready yet.
    """
    try:
        learner = get_learner(user_id)
        prediction = learner.predict_category(request.task.model_dump())

        if prediction is None:
            return PredictCategoryResponse(
                category=None,
                confidence=None,
                is_ready=False
            )

        category, confidence = prediction

        return PredictCategoryResponse(
            category=category,
            confidence=confidence,
            is_ready=True
        )

    except Exception as e:
        logger.error(f"Prediction error for user {user_id}: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/categorize/batch", response_model=BatchCategorizeResponse)
async def batch_categorize(
    request: BatchCategorizeRequest,
    user_id: str = Header(..., alias="X-User-Id")
):
    """
    Categorize multiple tasks (entire inbox) intelligently.

    Tasks with confidence >= threshold are auto-categorized.
    Tasks with confidence < threshold require manual confirmation
    but include suggestions.

    This endpoint is designed for "smart inbox processing":
    1. Auto-categorize high-confidence tasks (≥80%)
    2. Show suggestions for low-confidence tasks
    3. User confirms/corrects suggestions
    4. System learns from all actions
    """
    try:
        learner = get_learner(user_id)
        result = learner.batch_categorize(
            tasks=[t.model_dump() for t in request.tasks],
            confidence_threshold=request.confidence_threshold
        )

        # Convert to response models
        response = BatchCategorizeResponse(
            auto=[
                TaskWithCategory(
                    task=Task(**item['task']),
                    category=item['category'],
                    confidence=item['confidence']
                )
                for item in result['auto']
            ],
            manual=[
                TaskWithSuggestions(
                    task=Task(**item['task']),
                    suggestions=[
                        CategorySuggestion(**s)
                        for s in item['suggestions']
                    ]
                )
                for item in result['manual']
            ]
        )

        return response

    except Exception as e:
        logger.error(f"Batch categorize error for user {user_id}: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.get("/stats", response_model=LearningStatsResponse)
async def get_learning_stats(
    user_id: str = Header(..., alias="X-User-Id")
):
    """
    Get learning statistics for current user.

    Shows how many samples collected, categories learned, etc.
    """
    try:
        learner = get_learner(user_id)
        stats = learner.get_statistics()

        return LearningStatsResponse(**stats)

    except Exception as e:
        logger.error(f"Stats error for user {user_id}: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.delete("/reset")
async def reset_learning(
    user_id: str = Header(..., alias="X-User-Id")
):
    """
    Reset learning model for current user.

    Useful for testing or if user wants to start fresh.
    """
    try:
        learner = get_learner(user_id)

        # Reset using learner's method
        learner.reset()

        return {"message": "Learning model reset successfully"}

    except Exception as e:
        logger.error(f"Reset error for user {user_id}: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))
