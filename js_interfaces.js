mergeInto(LibraryManager.library, {
  play_audio: function(num) {
    var audio = []
    audio[0] = document.getElementById('audio_1')
    audio[1] = document.getElementById('audio_2')
    audio[2] = document.getElementById('audio_3')
    for (var i = 0; i < 3; i++) {
      if (i === num) {
        if (/iPhone|iPad|iPod/.test(navigator.userAgent)) {
          var f = function() {
            window.removeEventListener('touchstart', f, false)
            audio[num].play()
          }
          window.addEventListener('touchstart', f, false)
        } else {
          audio[i].play()
        }
      } else {
        audio[i].pause()
        audio[i].currentTime = 0;
      }
    }
  }
})
