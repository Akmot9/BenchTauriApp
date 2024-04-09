<template>
    <div>
      <h1>Liste des Fichiers Binaires</h1>
      <button @click="fetchBinaries">Rafraîchir la Liste</button>
      <input v-model="searchQuery" @input="filterBinaries" placeholder="Rechercher...">
      <button @click="benchmarkSelectedBinaries">Benchmark Sélectionnés</button>
      <table>
        <thead>
          <tr>
            <th>Sélection</th>
            <th>Nom du Fichier Binaire</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="binary in filteredBinaries" :key="binary">
            <td>
              <input type="checkbox" :value="binary" v-model="selectedBinaries">
            </td>
            <td>{{ binary }}</td>
          </tr>
        </tbody>
      </table>
      
    </div>
  </template>
  
  
  <script>
  import { invoke } from '@tauri-apps/api/tauri';
  
  export default {
    data() {
      return {
        binaries: [],
        filteredBinaries: [],
        selectedBinaries: [],
        searchQuery: ''
      };
    },
    methods: {
      async fetchBinaries() {
        try {
          const result = await invoke('list_binaries');
          this.binaries = result;
          this.filterBinaries();
        } catch (err) {
          console.error('Erreur lors de la récupération des binaires:', err);
        }
      },
      filterBinaries() {
        if (this.searchQuery) {
          this.filteredBinaries = this.binaries.filter(binary =>
            binary.toLowerCase().includes(this.searchQuery.toLowerCase())
          );
        } else {
          this.filteredBinaries = this.binaries;
        }
      },
      async benchmarkSelectedBinaries() {
        if (this.selectedBinaries.length === 0) {
          alert("Veuillez sélectionner au moins un binaire pour le benchmark.");
          return;
        }
        try {
          for (const binary of this.selectedBinaries) {
            // Remplacez 'benchmark_binary' par le nom de votre commande Tauri
            const result = await invoke('benchmark_binary', { binaries: this.selectedBinaries });
            console.log(`Résultat du benchmark pour ${binary}:`, result);
          }
          alert("Benchmark(s) completé(s). Voir la console pour les détails.");
        } catch (err) {
          console.error("Erreur lors du benchmark des binaires sélectionnés:", err);
          alert("Erreur lors du benchmark. Voir la console pour les détails.");
        }
      }
    }
  }
  </script>

  
  <style>
  /* Ajoutez votre style ici */
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    border: 1px solid #dddddd;
    text-align: left;
    padding: 8px;
  }
  
  tr:nth-child(even) {
    background-color: #363636;
  }
  </style>
  