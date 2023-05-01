use crate::app_state::AppState;
use axum::extract::State;

pub async fn get_user_profile(State(state): State<AppState>) {
    let access_code = state.get_google_access_code();
}
