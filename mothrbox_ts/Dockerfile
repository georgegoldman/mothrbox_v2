FROM denoland/deno:latest

# Create app directory
WORKDIR /app

# Copy application files (NOT .env - user provides it)
COPY deno.json ./
COPY src ./src


# Expose port
EXPOSE 8000

# Set default environment variables (can be overridden by user's .env)
ENV PORT=8000

# Run the application
# .env file is provided by user and mounted via docker-compose
CMD ["deno", "run", "--allow-net", "--allow-read", "--allow-env", "--allow-write", "--allow-sys", "--env-file", "src/main.ts"]