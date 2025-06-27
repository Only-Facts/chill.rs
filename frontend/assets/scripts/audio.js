const audio = document.getElementById('audio');
const playPause = document.getElementById('play-pause');
const playIcon = document.getElementById('playIcon');
const progress = document.getElementById('progress');
const progressContainer = document.getElementById('audio-progress');
const currentTimeSpan = document.getElementById('current-time');
const durationSpan = document.getElementById('duration');
const volumeSlider = document.getElementById('volume-slider');

volumeSlider.addEventListener('input', () => {
  audioPlayer.volume = volumeSlider.value;
});

function formatTime(sec) {
  const minutes = Math.floor(sec / 60);
  const seconds = Math.floor(sec % 60).toString().padStart(2, '0');
  return `${minutes}:${seconds}`;
}

playPause.addEventListener('click', () => {
  if (audio.paused) {
    audio.play();
    playIcon.innerHTML = `<path d="M6 4h4v16H6zm8 0h4v16h-4z"/>`; // Pause icon
  } else {
    audio.pause();
    playIcon.innerHTML = `<path d="M8 5v14l11-7z"/>`; // Play icon
  }
});

audio.addEventListener('timeupdate', () => {
  const progressPercent = (audio.currentTime / audio.duration) * 100;
  progress.style.width = `${progressPercent}%`;
  currentTimeSpan.textContent = formatTime(audio.currentTime);
});

audio.addEventListener('loadedmetadata', () => {
  durationSpan.textContent = formatTime(audio.duration);
});

progressContainer.addEventListener('click', (e) => {
  const rect = progressContainer.getBoundingClientRect();
  const clickX = e.clientX - rect.left;
  const width = rect.width;
  const seekTime = (clickX / width) * audio.duration;
  audio.currentTime = seekTime;
});
