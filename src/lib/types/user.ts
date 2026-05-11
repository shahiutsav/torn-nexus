export interface UserData {
  profile: Profile;
  bars: Bars;
  cooldowns: Cooldowns;
  icons: Icon[];
  battlestats: Battlestats;
  workstats: WorkStats;
  jobpoints: JobPoints;
  timestamp: number;
}

export interface Profile {
  id: number;
  name: string;
  level: number;
  rank: string;
  title: string;
  age: number;
  signed_up: number;
  faction_id: number;
  honor_id: number;
  property: Property;
  donator_status: string;
  image: string;
  gender: string;
  revivable: boolean;
  role: string;
  status: Status;
  spouse: Spouse;
  awards: number;
  friends: number;
  enemies: number;
  forum_posts: number;
  karma: number;
  last_action: LastAction;
  life: Life;
}

export interface Property {
  id: number;
  name: string;
}

export interface Status {
  description: string;
  details: string | null;
  state: string;
  color: string;
  until: number | null;
}

export interface Spouse {
  id: number;
  name: string;
  status: string;
  days_married: number;
}

export interface LastAction {
  status: string;
  timestamp: number;
  relative: string;
}

export interface Life {
  current: number;
  maximum: number;
}

export interface Bars {
  energy: Bar;
  nerve: Bar;
  happy: Bar;
  life: Bar;
  chain: Chain;
}

export interface Bar {
  current: number;
  maximum: number;
  increment: number;
  interval: number;
  tick_time: number;
  full_time: number;
}

export interface Chain {
  id: number;
  current: number;
  max: number;
  timeout: number;
  modifier: number;
  cooldown: number;
  start: number;
  end: number;
}

export interface Cooldowns {
  drug: number;
  medical: number;
  booster: number;
}

export interface Icon {
  id: number;
  title: string;
  description: string | null;
  until: number | null;
}

export interface Battlestats {
  strength: Battlestat;
  speed: Battlestat;
  defense: Battlestat;
  dexterity: Battlestat;
  total: number;
}

export interface Battlestat {
  value: number;
  modifier: number;
  modifiers: Modifier[];
}

export interface Modifier {
  effect: string;
  value: number;
  type: string;
}

export interface WorkStats {
  endurance: number;
  intelligence: number;
  manual_labor: number;
  total: number;
}

export interface JobPoints {
  jobs: Jobs;
  companies: CompanyPoints[];
}

export interface Jobs {
  army: number;
  casino: number;
  education: number;
  grocer: number;
  law: number;
  medical: number;
}

export interface CompanyPoints {
  company: JobPointsCompany;
  points: number;
}

export interface JobPointsCompany {
  id: number;
  name: String;
}
