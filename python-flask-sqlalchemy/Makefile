.ONESHELL:

venv:
	test -d venv || virtualenv venv

source: venv
	@. venv/bin/activate

install: source
	pip install -r requirements.txt 

setup: install

requirements.txt: source
	pip freeze > requirements.txt

run: source
	FLASK_ENV=development FLASK_APP=main.py flask run
