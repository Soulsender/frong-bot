docker build -t frong-bot .
docker run -ite $(cat .env | grep TOKEN) -v ./data:/app/data frong-bot