(function(){
	function pickRandom(arr){ return arr[Math.floor(Math.random()*arr.length)]; }

	function parseList(str){
		if(!str) return [];
		str = String(str).trim();
		// if it looks like a JSON array, parse it
		if(str.charAt(0) === '[' && str.charAt(str.length-1) === ']'){
			var parsed = JSON.parse(str);
			if(Array.isArray(parsed)){
				var out = [];
				for(var i=0;i<parsed.length;i++) out.push(String(parsed[i]));
				return out;
			}
		}
		var seps = [';','||',';;','\n','|',','];
		for(var si=0; si<seps.length; si++){
			var sep = seps[si];
			if(str.indexOf(sep) !== -1){
				var parts = str.split(sep);
				var out2 = [];
				for(var j=0;j<parts.length;j++){
					var t = parts[j].trim();
					if(t) out2.push(t);
				}
				return out2;
			}
		}
		return [str];
	}

	document.addEventListener('DOMContentLoaded', function(){
		var container = document.querySelector('.myPicture') || document.getElementById('my-picture') || document.body;
		
        var img = document.getElementById('my-picture-img') || container.querySelector('img');
		if(img){
			var facesList = parseList(container.getAttribute('data-faces'));
			var roll = Math.floor(Math.random()*10);
			if (roll === 0 && facesList.length > 0) {
				var file = pickRandom(facesList);
				img.src = '/assets/images/photos/faces/' + file;
			}
		}

		var quoteEl = document.querySelector('.PictureQuote') || container.querySelector('.PictureQuote');
		if(quoteEl){
			var raw = container.getAttribute('data-catchphrases') || quoteEl.getAttribute('data-catchphrases');
			var phrases = parseList(raw);
			if (phrases.length > 0) {
				var phrase = pickRandom(phrases);
				var prefix = "I'm Viveret and ";
				var text = phrase;
				if(String(phrase).toLowerCase().indexOf('viveret') === -1) text = prefix + phrase;
				quoteEl.textContent = text;
				quoteEl.style.opacity = 1;
			}
		}
	});

})();