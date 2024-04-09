<template>
    <div>
      <h1>Liste des Fichiers Binaires</h1>
      <button @click="fetchBinaries">Rafraîchir la Liste</button>
      <input v-model="searchQuery" @input="filterBinaries" placeholder="Rechercher...">
      <table>
        <tr>
          <th>Nom du Fichier Binaire</th>
        </tr>
        <tr v-for="binary in filteredBinaries" :key="binary">
          <td>{{ binary }}</td>
        </tr>
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
          this.binaries = [];
          this.filteredBinaries = [];
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
      }
    },
    // Fetch binaires when component is mounted
    mounted() {
      this.fetchBinaries();
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
  