BASE_URL='http://localhost:5000/'
BASE_GIF_URL="${BASE_URL}/gif/"
GET_GIF_URL="${BASE_GIF_URL}/1/"
AUTH_HEADER='Authorization: cloudbolt'
MIME_TYPE_HEADER='Content-Type: application/json'
NEW_GIF_DATA="{\"url\":\"https://some.random.gif/${RANDOM}.gif\"}"

echo "Initializing database request:"
curl -L --request POST --header "${MIME_TYPE_HEADER}" --header "${AUTH_HEADER}" ${BASE_URL}
echo "Creating gif request:"
curl -L --request POST --header "${MIME_TYPE_HEADER}" --header "${AUTH_HEADER}" --data "${NEW_GIF_DATA}" ${BASE_GIF_URL}
echo "Getting gif request:"
curl -L --request GET --header "${MIME_TYPE_HEADER}" ${GET_GIF_URL}
echo "Getting _all gifs_ request:"
curl -L --request GET --header "${MIME_TYPE_HEADER}" ${BASE_GIF_URL}
