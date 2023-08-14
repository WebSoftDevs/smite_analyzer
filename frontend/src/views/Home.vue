<template>
  <div class="home">
    <button @click="getMotds" class="btn-users">Fetch Motds </button>
  </div>
</template>

<script>

export default {
  name: "Home",
  data() {
    return {
      motds: [],
      
    };
  },
  methods: {
    getMotds() {
      const requestOptions = {
        method: 'GET',
        headers: { 'Content-Type': 'application/json'}
      };
      fetch('http://' + process.env.VUE_APP_BASE_URL + '/motd/get-all', requestOptions)
        .then(async response => {
          const data = await response.json();

          if (!response.ok) {
            const error = (data && data.message) || response.status;
            return Promise.reject(error);
          }
        })
        .catch(error => {
          console.error('There was an error!', error);
        });
    }
  }
}


</script>

<style scoped>
</style>