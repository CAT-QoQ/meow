import {createWebHistory, createRouter} from 'vue-router'

import IndexView from '../views/IndexView.vue';
import PreviewView from "../views/PreviewView.vue";

const routes = [
    {path: '/', component: IndexView},
    {path: '/preview', component: PreviewView},
]

export const router = createRouter({
    history: createWebHistory(),
    routes,
});