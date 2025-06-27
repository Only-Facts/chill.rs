const API_BASE_URL = "http://localhost:8081"
const audioPlayer = document.getElementById('audio');
const statusMessageElement = document.getElementById('status-message');
const prevBtn = document.getElementById("previous");
const nextBtn = document.getElementById("next");
const play = document.getElementById('playIcon');

let trackList = [];
let currentTrackIndex = 0;
let allPlaylists = {};
let currentPlaylist = [];
let currentPlaylistName = '';

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

function loadTrack(index) {
  const track = currentPlaylist[index];
  const streamUrl = `${API_BASE_URL}/music/${encodeURIComponent(track.path)}`;
  audioPlayer.src = streamUrl;
  audioPlayer.play();
  showStatus(`Playlist: ${currentPlaylistName} — Now playing: ${track.file}`);
}

function groupTracksByFolder(tracks) {
  const grouped = {};
  for (const track of tracks) {
    const [folder] = track.path.split('/');
    if (!grouped[folder]) grouped[folder] = [];
    grouped[folder].push(track);
  }
  return grouped;
}

function renderPlaylistSelector(playlists) {
  const menu = document.getElementById('playlist-menu');
  const toggleBtn = document.getElementById('playlist-toggle');

  menu.innerHTML = '';

  toggleBtn.onclick = () => {
    menu.classList.toggle('hidden');
  };

  Object.keys(playlists).forEach(name => {
    const item = document.createElement('li');
    item.textContent = name;
    item.onclick = () => {
      selectPlaylist(name);
      menu.classList.add('hidden');
      toggleBtn.textContent = `${name} ▾`;
    };
    menu.appendChild(item);
  });
}

function selectPlaylist(name) {
  currentPlaylistName = name;
  currentPlaylist = allPlaylists[name];
  currentTrackIndex = 0;
  loadTrack(currentTrackIndex);
  showStatus(`Playlist: ${name} — Now playing: ${currentPlaylist[0].file}`);
  play.innerHTML = `<path d="M6 4h4v16H6zm8 0h4v16h-4z"/>`;
}

async function fetchMusicList() {
  showStatus('Loading music...', 'loading');
  try {
    const response = await fetch(`${API_BASE_URL}/music/`);
    if (!response.ok) throw new Error(`HTTP error! Status: ${response.status}`);
    const tracks = await response.json();

    if (tracks.length === 0) {
      showStatus('No music found in the configured directory.', '');
    } else {
      allPlaylists = groupTracksByFolder(tracks);
      renderPlaylistSelector(allPlaylists);
      selectPlaylist(Object.keys(allPlaylists)[0]);
    }
  } catch (error) {
    console.error('Error fetching music list:', error);
    showStatus(`Failed to load music: ${error.message}`, 'error');
  }
}

prevBtn.addEventListener('click', () => {
  if (currentPlaylist.length === 0) return;
  currentTrackIndex = (currentTrackIndex - 1 + currentPlaylist.length) % currentPlaylist.length;
  loadTrack(currentTrackIndex);
});

nextBtn.addEventListener('click', () => {
  if (currentPlaylist.length === 0) return;
  currentTrackIndex = (currentTrackIndex + 1) % currentPlaylist.length;
  loadTrack(currentTrackIndex);
});

document.addEventListener('DOMContentLoaded', fetchMusicList);
