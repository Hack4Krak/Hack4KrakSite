# 🚀 Deployment

*It's time to ship it!*

## 🍖 Backend

We have [Dockerfile](Dockerfile) used to start a production server.

### Environment variables

- `BACKEND_ADDRESS` - address on which HTTP server should start
- `BACKEND_URL` - URL of the backend server
- `FRONTEND_URL` - URL of the frontend
- `DOMAIN` - production domain
- `COOKIES_DOMAIN` - domain used to set authorization cookies
- `RESEND_API_KEY` - API key for Resend email service
- `DATABASE_URL` - PostgreSQL connection string
- `JWT_SECRET` - secret used to sign JWT tokens

There are also environment variables related to downloading task repository:

- `GIT_TOKEN` - auth token used to authenticate to private repositories
- `GIT_USERNAME` - username used to authenticate to private repositories
- `TASKS_REPO_URL` - URL of the task repository

We also support support [Google](https://developers.google.com/identity/protocols/oauth2)
and [GitHub](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps) OAuth2 providers.

- `GOOGLE_CLIENT_ID` - Google OAuth2 client ID
- `GOOGLE_CLIENT_SECRET` - Google OAuth2 client secret
- `GITHUB_CLIENT_ID` - GitHub OAuth2 client ID
- `GITHUB_CLIENT_SECRET` - GitHub OAuth2 client secret

## 🥩 Frontend

You can refer to [Nuxt documentation](https://nuxt.com/docs/getting-started/deployment) for information on how to deploy
the frontend app.

### Environment variables

- `NUXT_SITE_URL` - Frontend website URL
- `BACKEND_ADDRESS` - address of the backend server (accessible from the frontend server and global network)