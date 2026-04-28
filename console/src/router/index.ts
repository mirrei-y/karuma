import { createRouter, createWebHistory } from "vue-router";
import SettingsView from "../views/Settings.vue";
import MessagesList from "../views/messages/List.vue";
import MessagesArchives from "../views/messages/Archives.vue";
import LoginView from "../views/Login.vue";
import PicturesView from "../views/Pictures.vue";

const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: "/login",
            name: "login",
            component: LoginView,
        },
        {
            path: "/",
            name: "settings",
            component: SettingsView,
            meta: { requiresAuth: true },
        },
        {
            path: "/messages",
            name: "messages-list",
            component: MessagesList,
            meta: { requiresAuth: true },
        },
        {
            path: "/messages/archives",
            name: "messages-archives",
            component: MessagesArchives,
            meta: { requiresAuth: true },
        },
        {
            path: "/pictures",
            name: "pictures",
            component: PicturesView,
        },
    ],
});

router.beforeEach((to, _from, next) => {
    if (to.meta.requiresAuth && !localStorage.getItem("karuma_passphrase")) {
        next({ name: "login" });
    } else {
        next();
    }
});

export default router;
