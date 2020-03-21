from markupsafe import escape
from flask import Flask, request, jsonify, abort
from flask_sqlalchemy import SQLAlchemy


PASSWORD="cloudbolt"
app = Flask(__name__)
app.config.update(
    SQLALCHEMY_DATABASE_URI="sqlite://",
    SQLALCHEMY_TRACK_MODIFICATIONS=False,
)
db = SQLAlchemy(app)


class Gif(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    url = db.Column(db.String(80), unique=False, nullable=False)

    def __repr__(self):
        return self.url


def request_password():
    return request.headers["Authorization"]

@app.route("/", methods=["POST"])
def index():
    if request_password() != PASSWORD:
        abort(401)
    db.create_all()
    return jsonify(status="Initialized Database!")


@app.errorhandler(404)
def notfound(_error):
    return jsonify(error="Thing not found!"), 404


@app.errorhandler(401)
def notfound(_error):
    return jsonify(error="You can't do that!"), 401


@app.route("/gif/", methods=["GET", "POST"])
def post_gif():
    if request.method == "POST":
        if request_password() != PASSWORD:
            abort(401)

        req = request.get_json()
        url = req.get("url")
        gif = Gif(url=url)

        db.session.add(gif)
        db.session.commit()
        resp = [{"id": gif.id, "url": gif.url}]
    elif request.method == "GET":
        gifs = Gif.query.all()
        resp = [{"id": gif.id, "url": gif.url} for gif in gifs]

    return jsonify(resp)


@app.route("/gif/<int:gif_id>/", methods=["GET"])
def get_gif(gif_id: int):
    gif = Gif.query.filter_by(id=gif_id).first()
    return jsonify(id=gif.id, url=gif.url)
