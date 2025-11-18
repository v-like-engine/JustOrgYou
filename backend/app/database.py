from motor.motor_asyncio import AsyncIOMotorClient, AsyncIOMotorDatabase
from typing import Optional
import logging

from app.config import settings

logger = logging.getLogger(__name__)

# Global database client
_client: Optional[AsyncIOMotorClient] = None
_db: Optional[AsyncIOMotorDatabase] = None


async def connect_db() -> None:
    """Connect to MongoDB."""
    global _client, _db

    try:
        _client = AsyncIOMotorClient(settings.MONGODB_URL)
        _db = _client[settings.MONGODB_DB_NAME]

        # Test connection
        await _client.admin.command("ping")
        logger.info(f"Connected to MongoDB: {settings.MONGODB_DB_NAME}")

    except Exception as e:
        logger.error(f"Failed to connect to MongoDB: {e}")
        raise


async def close_db() -> None:
    """Close MongoDB connection."""
    global _client

    if _client:
        _client.close()
        logger.info("MongoDB connection closed")


def get_database() -> AsyncIOMotorDatabase:
    """Get database instance."""
    if _db is None:
        raise RuntimeError("Database not initialized")
    return _db


async def get_collection(name: str):
    """Get a collection from the database."""
    db = get_database()
    return db[name]
