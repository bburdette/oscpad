var _bburdette$oscpad$Native_SvgTextSize = function() {
  
  function getTextWidth (t, f) {
    var blah = getTextMetrics(t, f);
    return blah.width; 
  };

  var getTextMetrics = function (text, font) {
     // re-use canvas object for better performance
     var canvas = getTextMetrics.canvas || (getTextMetrics.canvas = document.createElement("canvas"));
     var context = canvas.getContext("2d");
     context.font = font;
     var metrics = context.measureText(text);
     return metrics;
    };
  
  return {
    getTextWidth: F2(getTextWidth)
  };

  
}();


