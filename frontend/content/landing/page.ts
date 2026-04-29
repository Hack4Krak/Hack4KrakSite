import LANDING_PAGE from './page.json'

export type LandingPageContent = typeof LANDING_PAGE
export type LandingVenue = LandingPageContent['event']['venue']

export default LANDING_PAGE
