from app.api.health import router as health_router
from app.api.ai import router as ai_router
from app.api.learning import router as learning_router
from app.api.search import router as search_router

__all__ = ["health_router", "ai_router", "learning_router", "search_router"]
