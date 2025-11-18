from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from typing import List, Optional
import logging

from app.services.ml import (
    semantic_search,
    suggest_tags,
    auto_categorize,
    detect_duplicates,
)

logger = logging.getLogger(__name__)
router = APIRouter()


# Request/Response Models
class Task(BaseModel):
    """Task model for API."""

    title: str
    body: Optional[str] = None
    tags: List[str] = []
    keyword: str = "TODO"


class SemanticSearchRequest(BaseModel):
    """Semantic search request."""

    query: str
    tasks: List[Task]
    limit: int = 10


class SemanticSearchResponse(BaseModel):
    """Semantic search response."""

    results: List[dict]


class TagSuggestionRequest(BaseModel):
    """Tag suggestion request."""

    task: Task
    existing_tags: List[str] = []
    limit: int = 5


class TagSuggestionResponse(BaseModel):
    """Tag suggestion response."""

    suggested_tags: List[str]


class AutoCategorizeRequest(BaseModel):
    """Auto categorization request."""

    task: Task


class AutoCategorizeResponse(BaseModel):
    """Auto categorization response."""

    suggested_keyword: str
    confidence: float


class DuplicateDetectionRequest(BaseModel):
    """Duplicate detection request."""

    tasks: List[Task]
    threshold: float = 0.85


class DuplicateDetectionResponse(BaseModel):
    """Duplicate detection response."""

    duplicates: List[dict]


# AI Endpoints
@router.post("/search/semantic", response_model=SemanticSearchResponse)
async def search_tasks_semantic(request: SemanticSearchRequest):
    """
    Perform semantic search on tasks using ML embeddings.

    This allows searching by meaning rather than exact keywords.
    """
    try:
        results = await semantic_search(
            query=request.query,
            tasks=[t.model_dump() for t in request.tasks],
            limit=request.limit,
        )

        return SemanticSearchResponse(results=results)

    except Exception as e:
        logger.error(f"Semantic search error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/tags/suggest", response_model=TagSuggestionResponse)
async def suggest_task_tags(request: TagSuggestionRequest):
    """
    Suggest relevant tags for a task based on its content.

    Uses ML to analyze task title and body to suggest appropriate tags.
    """
    try:
        suggested_tags = await suggest_tags(
            task=request.task.model_dump(),
            existing_tags=request.existing_tags,
            limit=request.limit,
        )

        return TagSuggestionResponse(suggested_tags=suggested_tags)

    except Exception as e:
        logger.error(f"Tag suggestion error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/categorize", response_model=AutoCategorizeResponse)
async def categorize_task(request: AutoCategorizeRequest):
    """
    Automatically categorize a task (suggest keyword/state).

    Analyzes task content to suggest appropriate category (TODO, WAITING, etc.).
    """
    try:
        keyword, confidence = await auto_categorize(
            task=request.task.model_dump()
        )

        return AutoCategorizeResponse(
            suggested_keyword=keyword,
            confidence=confidence,
        )

    except Exception as e:
        logger.error(f"Auto categorization error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/duplicates/detect", response_model=DuplicateDetectionResponse)
async def detect_duplicate_tasks(request: DuplicateDetectionRequest):
    """
    Detect duplicate or very similar tasks.

    Uses ML similarity to find tasks that might be duplicates.
    """
    try:
        duplicates = await detect_duplicates(
            tasks=[t.model_dump() for t in request.tasks],
            threshold=request.threshold,
        )

        return DuplicateDetectionResponse(duplicates=duplicates)

    except Exception as e:
        logger.error(f"Duplicate detection error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))
