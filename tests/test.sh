#!/bin/bash

BASE_URL='http://localhost:5000/'
BASE_GIF_URL="${BASE_URL}gif/"
GET_GIF_URL="${BASE_GIF_URL}1/"
AUTH_HEADER='Authorization: cloudbolt'
MIME_TYPE_HEADER='Content-Type: application/json'
NEW_GIF_DATA="{\"url\":\"https://some.random.gif/${RANDOM}.gif\"}"

VERBOSE=$1
if [[ $VERBOSE != "-v" ]]; then
    VERBOSE=""
fi

echo "Initializing database request"
test $VERBOSE && \
    echo curl -s -L --request POST --header "${MIME_TYPE_HEADER}" --header "${AUTH_HEADER}" ${BASE_URL} | jq .
curl -s -L --request POST --header "${MIME_TYPE_HEADER}" --header "${AUTH_HEADER}" ${BASE_URL} | jq .
echo


echo "Creating gif request"
test $VERBOSE && \
    echo curl -s -L --request POST --header "${MIME_TYPE_HEADER}" --header "${AUTH_HEADER}" --data "${NEW_GIF_DATA}" ${BASE_GIF_URL} | jq .
curl -s -L --request POST --header "${MIME_TYPE_HEADER}" --header "${AUTH_HEADER}" --data "${NEW_GIF_DATA}" ${BASE_GIF_URL} | jq .
echo


echo "Getting gif request"
test $VERBOSE && \
    echo curl -s -L --request GET --header "${MIME_TYPE_HEADER}" ${GET_GIF_URL} | jq .
curl -s -L --request GET --header "${MIME_TYPE_HEADER}" ${GET_GIF_URL} | jq .
echo


echo "Getting _all gifs_ request"
test $VERBOSE && \
    echo curl -s -L --request GET --header "${MIME_TYPE_HEADER}" ${BASE_GIF_URL} | jq .
curl -s -L --request GET --header "${MIME_TYPE_HEADER}" ${BASE_GIF_URL} | jq .
echo
