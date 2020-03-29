<template>
  <div>
    <div>
      <input v-model="mailAddress" placeholder="メールアドレス">
    </div>

    <div>
      <input v-model="password" placeholder="パスワード">
    </div>

    <button @click="login()">メールアドレス&パスワードで新規登録</button>
  </div>
</template>

<script lang="ts">
  import firebase from "firebase";
  import {Component, Vue} from "vue-property-decorator";

  @Component
  export default class MailAddressLoginForm extends Vue {

    private mailAddress: string | null = null;
    private password: string | null = null;

    private login() {
      if(this.mailAddress != null && this.password != null) {
        firebase.auth().createUserWithEmailAndPassword(this.mailAddress, this.password)
          .then(() => {
            alert("新規ユーザの作成に成功")
          })
          .catch(e => console.error(e.toString()));
      } else {
        alert("メールアドレスとパスワードを指定してください");
      }
    }
  }
</script>
