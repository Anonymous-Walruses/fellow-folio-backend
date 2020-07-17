module.exports = (db) => {
  var express = require('express');
  var router = express.Router();

  /* GET home page. */
  router.get('/user', async function (req, res, next) {
    const doc = await db.collection('users').doc(req.query.id).get();
    res.send(doc.data());
  });

  return router

}
