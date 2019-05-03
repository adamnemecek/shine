var Shine = function () {
	return {
		notifyUser: function () {
			var xhr = new XMLHttpRequest();
			xhr.open("POST", "rest/v1/control/notify", true);
			xhr.setRequestHeader('Content-Type', 'application/json');
			xhr.send(JSON.stringify({}));
		}
	}
};

"object" === typeof module && (module.exports = Shine);