import Vue from 'vue'
import firebase from 'firebase';
import credentials from './conf/credentials.json';

import App from './App.vue'
import router from './router'

Vue.config.productionTip = false;
firebase.initializeApp(credentials);

new Vue({
  router,
  render: h => h(App)
}).$mount('#app');
