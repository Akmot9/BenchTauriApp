<template>
    <div>
      <canvas id="memoryChart"></canvas>
      <button @click="loadData">Load Data</button>
    </div>
  </template>
  
  <script>
  import { invoke } from '@tauri-apps/api';
  import { Chart, registerables } from 'chart.js';
  Chart.register(...registerables);
  
  export default {
    methods: {
        loadData() {
  invoke('get_memory_usage')
    .then((data) => {
      const parsedData = data.map(line => {
        const parts = line.split(', ');
        // S'assurer qu'il y a bien trois parties et que parts[2] existe
        if (parts.length === 3 && parts[2]) {
          const second = parseInt(parts[0].replace('s', ''), 10);
          const memory = parseInt(parts[2].replace(' KB', ''), 10);
          return { second, memory };
        }
        return null;
      }).filter(d => d !== null); // Filtrer les entrées nulles

      const labels = parsedData.map(d => `${d.second}s`);
      const memoryData = parsedData.map(d => d.memory);

      new Chart(document.getElementById('memoryChart'), {
        type: 'line',
        data: {
          labels: labels,
          datasets: [{
            label: 'Memory Usage (KB)',
            data: memoryData,
            fill: false,
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1
          }]
        }
      });
    })
    .catch(console.error);
}
  }}
  </script>
  