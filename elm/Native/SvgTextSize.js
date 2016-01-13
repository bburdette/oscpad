Elm.Native.SvgTextSize = {};
Elm.Native.SvgTextSize.make = function(localRuntime) {
  localRuntime.Native = localRuntime.Native || {};
  localRuntime.Native.SvgTextSize = localRuntime.Native.SvgTextSize || {};
  if (localRuntime.Native.SvgTextSize.values) {
    return localRuntime.Native.SvgTextSize.values;
  }

  // var Signal = Elm.Native.Signal.make(localRuntime);
  var Task = Elm.Native.Task.make(localRuntime);
  var Utils = Elm.Native.Utils.make(localRuntime);

  var getTInt = Task.asyncFunction(function(callback) {

      console.log("meh");
      // console.log("meh" + textsizerequest);
      //
      // -- blah = getTextMetrics(textsizerequest.text, "");
      // textbounds = { w: blah.width, h: blah.height };  
      // textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      return callback(Task.succeed(5));
    });
  // };

  /*
  function getTInt() {
     return Task.asyncFunction(function(callback) {

      console.log("meh");
      // console.log("meh" + textsizerequest);
      //
      // -- blah = getTextMetrics(textsizerequest.text, "");
      // textbounds = { w: blah.width, h: blah.height };  
      // textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      return callback(Task.succeed(5));
    });
  }
  // };
  */

  var getTb = Task.asyncFunction(function(callback) {

      console.log("meh");
      // console.log("meh" + textsizerequest);
      //
      // -- blah = getTextMetrics(textsizerequest.text, "");
      // textbounds = { w: blah.width, h: blah.height };  
      textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      callback(Task.succeed(textbounds));
    });
  // };

  var getTextSize = function (textsizerequest) {
    return Task.asyncFunction(function(callback) {

      // console.log("meh");
      // console.log("meh" + textsizerequest.text);
      //
      blah = getTextMetrics(textsizerequest.text, textsizerequest.font);
      textbounds = { w: blah.width, h: blah.width };  
      // textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      callback(Task.succeed(textbounds));
    });
  };

   var getTextWidth = function (t, f) {
    return Task.asyncFunction(function(callback) {

      // console.log("meh");
      // console.log("meh" + textsizerequest.text);
      //
      blah = getTextMetrics(t, f); 
      // textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      callback(Task.succeed(blah.width));
    });
  };

  var getTextWidthNow = function (t, f) {
    blah = getTextMetrics(t, f);
    return blah.width; 
  };

  /*
  var getTextWidth = function (textsizerequest) {
    return Task.asyncFunction(function(callback) {

      // console.log("meh");
      // console.log("meh" + textsizerequest.text);
      //
      blah = getTextMetrics(textsizerequest.text, textsizerequest.font);
      // textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      callback(Task.succeed(blah.width));
    });
  };

  var getTextWidth = function (text, font) {
    return Task.asyncFunction(function(callback) {

      console.log("textwidth");
      // console.log("meh" + textsizerequest.text);
      //
      mtrx = getTextMetrics(text, font);
      textbounds = { w: blah.width, h: blah.width };  
      // textbounds = { w: blah.width, h: blah.height };  
      // textbounds = { w: 100, h: 100 };  

      // Tast.perform(address._0(textbounds));

      callback(Task.succeed(textbounds));
    });
  };
  */

  /**
   * Uses canvas.measureText to compute and return the width of the given text of given font in pixels.
   * 
   * @param {String} text The text to be rendered.
   * @param {String} font The css font descriptor that text is to be rendered with (e.g. "bold 14px verdana").
   * 
   * see http://stackoverflow.com/questions/118241/calculate-text-width-with-javascript/21015393#21015393
   */
  var getTextMetrics = function (text, font) {
        // re-use canvas object for better performance
     var canvas = getTextMetrics.canvas || (getTextMetrics.canvas = document.createElement("canvas"));
     var context = canvas.getContext("2d");
     context.font = font;
     var metrics = context.measureText(text);
     return metrics;
    };
  
  /*  
  localRuntime.Native.SvgTextSize.values = {
     getTextSize: F2(getTextSize)
  };
  */

  var getCurrentTime = Task.asyncFunction(function(callback) {
		return callback(Task.succeed(Date.now()));
	});

  return localRuntime.Native.SvgTextSize.values = 
  {
     getTextWidth: F2(getTextWidth),
     getTextWidthNow: F2(getTextWidthNow),
     getTextSize: getTextSize,
     getTInt: getTInt,
     getTb: getTb,
     getTextMetrics: getTextMetrics,
     getCurrentTime: getCurrentTime
  }
}
