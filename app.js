var express = require('express');
var path = require('path');
var cookieParser = require('cookie-parser');
var logger = require('morgan');


var app = express();

var admin = require('firebase-admin');

var serviceAccount = require(path.join(__dirname, 'service-account.json'));
admin.initializeApp({
    credential: admin.credential.cert(serviceAccount),
    databaseURL: "https://fellowfolio.firebaseio.com"
});
var db = admin.firestore();

var indexRouter = require('./routes/index')(db);
var usersRouter = require('./routes/users');

app.use(logger('dev'));
app.use(express.json());
app.use(express.urlencoded({ extended: false }));
app.use(cookieParser());
app.use(express.static(path.join(__dirname, 'public')));

app.use('/api/v001/', indexRouter);

module.exports = app;
