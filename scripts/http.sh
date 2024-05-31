curl -X POST http://localhost:8000/subscriptions \
   -H "Content-Type: application/x-www-form-urlencoded" \
   -d "name=$1&email=$2"
