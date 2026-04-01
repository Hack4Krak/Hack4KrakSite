export interface OrganizerProps {
  name: string
  role: string
  linkedinUrl: string
}

export const organizers: OrganizerProps[] = [
  {
    name: 'Jakub Starzyk',
    role: 'Head Coordinator',
    linkedinUrl: 'https://www.linkedin.com/in/jakub-starzyk-8a37bb331/',
  },
  {
    name: 'Norbert Szeremet',
    role: 'Lead IT Coordinator',
    linkedinUrl: 'https://www.linkedin.com/in/norbert-szeremet/',
  },
  // Add more organizers here
]
