from pydantic_settings import BaseSettings
from typing import List
import os


class Settings(BaseSettings):
    """Application settings."""

    # API Settings
    API_V1_STR: str = "/api/v1"
    PROJECT_NAME: str = "JustOrgYou"
    DEBUG: bool = True

    # CORS Settings
    CORS_ORIGINS: List[str] = [
        "http://localhost:3000",
        "http://localhost:8080",
        "http://localhost:5173",
    ]

    # Database Settings
    MONGODB_URL: str = "mongodb://localhost:27017"
    MONGODB_DB_NAME: str = "justorgyou"

    # Security Settings
    SECRET_KEY: str = "your-secret-key-here-change-in-production"
    ALGORITHM: str = "HS256"
    ACCESS_TOKEN_EXPIRE_MINUTES: int = 30

    # AI/ML Settings
    ML_MODEL_PATH: str = "./models"
    SENTENCE_TRANSFORMER_MODEL: str = "all-MiniLM-L6-v2"
    EMBEDDING_DIM: int = 384

    # File Settings
    MAX_UPLOAD_SIZE: int = 10 * 1024 * 1024  # 10MB
    ALLOWED_EXTENSIONS: List[str] = [".org", ".md", ".txt"]

    class Config:
        env_file = ".env"
        case_sensitive = True


settings = Settings()
