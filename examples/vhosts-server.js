var express = require("express");
var app = express();

var DEFAULT = "Hello World!";
var vhosts = ["1.test.local", "10.test.local", "15.test.local"];

app.all("/*", function (req, res) {
    if (vhosts.some(x => x === req.hostname)) {
        return res.send(req.hostname);
    }

    res.send("Hello World!");
});

app.listen(3000, function () {
    console.log("Example app listening on port 3000!");
});

