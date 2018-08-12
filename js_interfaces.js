mergeInto(LibraryManager.library, {
  play_audio: function(num) {
    var audio = []
    audio[0] = document.getElementById('audio_1')
    audio[1] = document.getElementById('audio_2')
    audio[2] = document.getElementById('audio_3')
    for (var i = 0; i < 3; i++) {
      if (i === num) {
        audio[i].play()
      } else {
        audio[i].pause()
        audio[i].currentTime = 0;
      }
    }
  }
})
