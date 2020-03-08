<template>
    <button @click="onClick()">ログアウト</button>
</template>

<script lang="ts">
    import firebase from "firebase";
    import {Component, Vue} from "vue-property-decorator";

    @Component
    export default class LogoutForm extends Vue {
        private onClick() {
            firebase.auth().signOut()
              .then(() => alert('ログアウトに成功'))
              .catch(e => console.error(e));
        }
    }
</script>
