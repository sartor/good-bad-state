<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js App"/>
    <p>From server:</p>
    <p>{{ resp }}</p>
  </div>
</template>

<script>
// @ is an alias to /src
import HelloWorld from '@/front/components/HelloWorld.vue'

export default {
    components: {
        HelloWorld
    },
    data() {
        return {
            resp: '',
        }
    },
    methods: {
        load() {
            this.$fetch('hello').then(resp => this.resp = resp);
        },

    },
    mounted() {
        this.load();
    }
}
</script>
