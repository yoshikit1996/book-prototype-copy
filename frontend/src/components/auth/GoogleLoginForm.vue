<template>
    <button @click="onClick()">Googleでログイン/新規登録</button>
</template>

<script lang="ts">
    import firebase from "firebase";
    import {Component, Vue} from "vue-property-decorator";

    @Component
    export default class GoogleLoginForm extends Vue {
        private onClick() {
            const provider = new firebase.auth.GoogleAuthProvider();
            firebase.auth().signInWithPopup(provider);
        }
    }
</script>
