from fastapi import APIRouter, HTTPException, Header
from pydantic import BaseModel
from typing import List, Optional, Dict, Any
import logging

from app.services.ml import semantic_search

logger = logging.getLogger(__name__)
router = APIRouter()


class Task(BaseModel):
    """Task model for API."""
    id: Optional[str] = None
    title: str
    body: Optional[str] = None
    tags: List[str] = []
    keyword: str = "TODO"


class SemanticSearchRequest(BaseModel):
    """Semantic search request across all tasks."""
    query: str
    tasks: List[Task]
    limit: int = 20


class SearchResult(BaseModel):
    """Single search result with task and score."""
    task: Task
    score: float
    matched_text: str


class SemanticSearchResponse(BaseModel):
    """Semantic search response."""
    query: str
    results: List[SearchResult]
    total_searched: int


@router.post("/semantic", response_model=SemanticSearchResponse)
async def search_tasks_semantic(request: SemanticSearchRequest):
    """
    Perform semantic search across ALL tasks (all directories/categories).

    This endpoint searches by meaning, not just keywords:
    - "Buy bread" will find "Visit local supermarket to buy milk, loaf and cheese"
    - "Website tasks" will find "Fix homepage bug" and "Update landing page"
    - "Urgent work" will find tasks tagged with :work:urgent: even without those exact words

    The search uses AI embeddings to understand semantic similarity.

    Example usage:
    ```
    POST /api/v1/search/semantic
    {
      "query": "buy bread",
      "tasks": [...all your tasks from all categories...],
      "limit": 20
    }
    ```

    Returns tasks sorted by relevance with confidence scores.
    """
    try:
        # Perform semantic search
        raw_results = await semantic_search(
            query=request.query,
            tasks=[t.model_dump() for t in request.tasks],
            limit=request.limit,
        )

        # Format results
        results = []
        for item in raw_results:
            task_data = item['task']
            score = item['score']

            # Extract matched text for highlighting
            matched_text = _extract_matched_text(task_data, request.query)

            results.append(
                SearchResult(
                    task=Task(**task_data),
                    score=score,
                    matched_text=matched_text
                )
            )

        return SemanticSearchResponse(
            query=request.query,
            results=results,
            total_searched=len(request.tasks)
        )

    except Exception as e:
        logger.error(f"Semantic search error: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))


def _extract_matched_text(task: Dict[str, Any], query: str) -> str:
    """
    Extract the most relevant text snippet from task.

    This helps highlight what matched in the UI.
    """
    # Simple heuristic: return title, or first line of body if title is short
    title = task.get('title', '')

    if len(title) > 30:
        return title

    body = task.get('body', '')
    if body:
        first_line = body.split('\n')[0]
        return f"{title} - {first_line[:100]}"

    return title


@router.get("/suggest-query")
async def suggest_search_queries(
    user_id: str = Header(..., alias="X-User-Id")
):
    """
    Suggest common search queries based on user's tasks.

    Returns helpful search suggestions like:
    - "overdue tasks"
    - "high priority work"
    - "waiting tasks"
    etc.
    """
    # TODO: Implement smart suggestions based on user's common patterns
    suggestions = [
        "overdue tasks",
        "high priority",
        "work tasks",
        "personal tasks",
        "quick tasks",
        "deep work",
        "calls to make",
        "emails to send",
        "errands to run",
        "meetings this week"
    ]

    return {
        "suggestions": suggestions
    }
