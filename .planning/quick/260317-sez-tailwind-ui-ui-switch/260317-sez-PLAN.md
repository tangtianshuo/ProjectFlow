---
phase: quick
plan: "260317-sez"
type: execute
wave: 1
depends_on: []
files_modified:
  - src/components/ui/ThemeSwitch.vue
  - src/components/layout/Sidebar.vue
  - src/style.css
autonomous: true
requirements: []
user_setup: []
---

<objective>
Create an animated theme switch component using Tailwind UI-style toggle, replacing the existing icon-based theme toggle.

Purpose: Provide a visually appealing toggle switch for dark/light theme with smooth animation.
Output: ThemeSwitch.vue component with animated toggle, integrated into Sidebar.
</objective>

<execution_context>
@D:\Projects\Tauri\ProjectFlow\src\style.css
@D:\Projects\Tauri\ProjectFlow\src\components\layout\Sidebar.vue
</execution_context>

<context>
The project already has:
- Tailwind CSS v4 installed
- CSS variables for dark/light themes in style.css
- Theme toggle logic in uiStore (toggleTheme, theme state)
- Existing icon-based toggle in Sidebar

The new toggle should:
- Be a slider/toggle switch style (not icon button)
- Have smooth animation when switching
- Show sun icon on one side, moon on the other
- Use CSS variables to adapt to light/dark theme
</context>

<tasks>

<task type="auto">
  <name>Task 1: Create ThemeSwitch component</name>
  <files>src/components/ui/ThemeSwitch.vue</files>
  <action>
    Create a new ThemeSwitch component with the following:
    - Use Vue 3 composition API with useUiStore
    - Toggle switch with sliding circle animation
    - Sun icon on right (light mode), Moon icon on left (dark mode)
    - Smooth transition animation (300ms) using CSS transforms
    - Use CSS variables for colors: --bg-tertiary for track, --accent-primary for active state
    - Emit click event or call uiStore.toggleTheme directly
    - Add dark/light mode visual states:
      - Dark: darker track, moon icon visible
      - Light: lighter track with accent color, sun icon visible
  </action>
  <verify>
    <automated>File exists: src/components/ui/ThemeSwitch.vue</automated>
  </verify>
  <done>ThemeSwitch.vue created with animated toggle functionality</done>
</task>

<task type="auto">
  <name>Task 2: Integrate ThemeSwitch into Sidebar</name>
  <files>src/components/layout/Sidebar.vue</files>
  <action>
    Replace existing icon-based theme toggle buttons with ThemeSwitch component:
    1. Import ThemeSwitch from '../ui/ThemeSwitch.vue'
    2. Replace mobile toggle (line 34-41) with ThemeSwitch
    3. Replace desktop toggle (line 113-124) with ThemeSwitch
    4. Remove the old @click="uiStore.toggleTheme" buttons
    5. ThemeSwitch handles its own toggle logic
  </action>
  <verify>
    <automated>grep -n "ThemeSwitch" src/components/layout/Sidebar.vue</automated>
  </verify>
  <done>Sidebar uses ThemeSwitch component in both mobile and desktop views</done>
</task>

<task type="auto">
  <name>Task 3: Verify theme transition animation</name>
  <files>src/style.css</files>
  <action>
    Ensure CSS transitions are smooth for theme changes:
    1. Verify existing transition rules (line 3-9 in style.css)
    2. Add specific transition for ThemeSwitch track and circle if needed
    3. Ensure all theme-related elements use CSS variables
  </action>
  <verify>
    <automated>grep -n "transition" src/style.css | head -10</automated>
  </verify>
  <done>Theme transitions are smooth with animation</done>
</task>

</tasks>

<verification>
- ThemeSwitch component renders correctly
- Toggle animation works when switching themes
- Both mobile and desktop views show the toggle switch
- Theme persists after page refresh
</verification>

<success_criteria>
- Animated toggle switch visible in Sidebar
- Clicking toggle switches between dark/light theme
- Animation is smooth (300ms transition)
- Theme preference persists in localStorage
</success_criteria>

<output>
After completion, create .planning/quick/260317-sez-tailwind-ui-ui-switch/260317-sez-SUMMARY.md
</output>
