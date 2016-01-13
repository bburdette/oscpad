Elm.Native.SvgTextSize = {};
Elm.Native.SvgTextSize.make = function(localRuntime) {
  localRuntime.Native = localRuntime.Native || {};
  localRuntime.Native.SvgTextSize = localRuntime.Native.SvgTextSize || {};
  if (localRuntime.Native.SvgTextSize.values) {
    return localRuntime.Native.SvgTextSize.values;
  }

  // var Signal = Elm.Native.Signal.make(localRuntime);
  // var Task = Elm.Native.Task.make(localRuntime);
  // var Utils = Elm.Native.Utils.make(localRuntime);

  var getTextWidth = function (t, f) {
    blah = getTextMetrics(t, f);
    return blah.width; 
  };

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
  
  return localRuntime.Native.SvgTextSize.values = 
  {
     getTextWidth: F2(getTextWidth),
  }
}
