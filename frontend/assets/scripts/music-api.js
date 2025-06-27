const API_BASE_URL = "http://localhost:8081"
const audioPlayer = document.getElementById('audio');
const statusMessageElement = document.getElementById('status-message');
const prevBtn = document.getElementById("previous");
const nextBtn = document.getElementById("next");
const play = document.getElementById('playIcon');

let trackList = [];
let currentTrackIndex = 0;

function showStatus(message, type = '') {
  statusMessageElement.textContent = message;
  statusMessageElement.className = `status-message ${type}-message`;
  if (type === 'error') {
    statusMessageElement.classList.add('error-message');
  } else if (type === 'loading') {
    statusMessageElement.classList.add('loading-message');
  } else {
    statusMessageElement.classList.remove('error-message', 'loading-message');
  }
}

async function fetchMusicList() {
  showStatus('Loading music...', 'loading');
  try {
    const response = await fetch(`${API_BASE_URL}/music/`);
    if (!response.ok) {
      throw new Error(`HTTP error! Status: ${response.status}`);
    }
    const tracks = await response.json();
    if (tracks.length === 0) {
      showStatus('No music found in the configured directory.', '');
    } else {
      trackList = tracks;
      currentTrackIndex = 0;
      loadTrack(currentTrackIndex);
      showStatus(`Now playing: ${trackList[currentTrackIndex].file}`, '');
    }
  } catch (error) {
    console.error('Error fetching music list:', error);
    showStatus(`Failed to load music: ${error.message}`, 'error');
  }
}

function loadTrack(index) {
  const track = trackList[index];
  const streamUrl = `${API_BASE_URL}/music/${encodeURIComponent(track.path)}`;
  audioPlayer.src = streamUrl;
  audioPlayer.play();
  showStatus(`Now playing: ${track.file}`, '');
}

prevBtn.addEventListener('click', () => {
  if (trackList.length === 0) return;
  currentTrackIndex = (currentTrackIndex - 1 + trackList.length) % trackList.length;
  loadTrack(currentTrackIndex);
  play.innerHTML = `<path d="M6 4h4v16H6zm8 0h4v16h-4z"/>`;
});

nextBtn.addEventListener('click', () => {
  if (trackList.length === 0) return;
  currentTrackIndex = (currentTrackIndex + 1) % trackList.length;
  loadTrack(currentTrackIndex);
  play.innerHTML = `<path d="M6 4h4v16H6zm8 0h4v16h-4z"/>`;
});

document.addEventListener('DOMContentLoaded', fetchMusicList);
