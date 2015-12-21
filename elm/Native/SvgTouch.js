Elm.Native = Elm.Native || {};
Elm.Native.SvgTouch = {};
Elm.Native.SvgTouch.make = function(localRuntime) {
	localRuntime.Native = localRuntime.Native || {};
	localRuntime.Native.SvgTouch = localRuntime.Native.SvgTouch || {};
	if (localRuntime.Native.SvgTouch.values)
	{
		return localRuntime.Native.SvgTouch.values;
	}

	var Signal = Elm.Signal.make(localRuntime);
	var NS = Elm.Native.Signal.make(localRuntime);
	var List = Elm.Native.List.make(localRuntime);
	var Utils = Elm.Native.Utils.make(localRuntime);

	function Dict() {
		this.keys = [];
		this.values = [];

		this.insert = function(key, value) {
			this.keys.push(key);
			this.values.push(value);
		};
		this.lookup = function(key) {
			var i = this.keys.indexOf(key);
			return i >= 0 ? this.values[i] : {x: 0, y: 0, t: 0};
		};
		this.remove = function(key) {
			var i = this.keys.indexOf(key);
			if (i < 0) return;
			var t = this.values[i];
			this.keys.splice(i, 1);
			this.values.splice(i, 1);
			return t;
		};
		this.clear = function() {
			this.keys = [];
			this.values = [];
		};
	}

	var root = NS.input('touch', []),
		tapTime = 500,
		hasTap = false,
		tap = { x: 0, y: 0},
		dict = new Dict();

	function touch(t) {
		var r = dict.lookup(t.identifier);
		var point = Utils.getXY(t);
		return {
			id: t.identifier,
			x: point._0,
			y: point._1,
			x0: r.x,
			y0: r.y,
			t0: r.t
		 };
	}

	var node = localRuntime.isFullscreen()
		? document
		: localRuntime.node;

	function start(e) {
		var point = Utils.getXY(e);
		dict.insert(e.identifier, {
			x: point._0,
			y: point._1,
			t: localRuntime.timer.now()
		});
	}
	function end(e) {
		var t = dict.remove(e.identifier);
		if (typeof t === "undefined") {
			return;
		}
		if (localRuntime.timer.now() - t.t < tapTime)
		{
			hasTap = true;
			tap = {
				x: t.x,
				y: t.y
			};
		}
	}

	function listen(name, f) {
		function update(e) {
			for (var i = e.changedTouches.length; i--; ) {
				f(e.changedTouches[i]);
			}
			var ts = new Array(e.touches.length);
			for (var i = e.touches.length; i--; ) {
				ts[i] = touch(e.touches[i]);
			}
			localRuntime.notify(root.id, ts);
			e.preventDefault();
		}
		localRuntime.addListener([root.id], node, name, update);
	}

	listen('touchstart', start);
	listen('touchmove', function(_) {});
	listen('touchend', end);
	listen('touchcancel', end);
	listen('touchleave', end);

	function dependency(f) {
		var sig = A2( Signal.map, f, root );
		root.defaultNumberOfKids += 1;
		sig.defaultNumberOfKids = 0;
		return sig;
	}

	var touches = dependency(List.fromArray);

	var taps = function() {
		var sig = dependency(function(_) { return tap; });
		sig.defaultNumberOfKids = 1;
		function pred(_) {
			var b = hasTap;
			hasTap = false;
			return b;
		}
		var sig2 = A3(Signal.filter, pred, {x: 0, y: 0}, sig);
		sig2.defaultNumberOfKids = 0;
		return sig2;
	}();

	return localRuntime.Native.SvgTouch.values = { touches: touches, taps: taps };
};
