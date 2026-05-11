# Expo Official Skills Playbook

This README explains how to use the official Expo Skills for React Native + Expo projects.

The goal is to use Expo’s official AI agent skills directly whenever possible instead of recreating
or merging them into a custom skill. Expo maintains these skills specifically for building,
deploying, and debugging Expo and React Native apps.

## Principle

Use official Expo Skills as the primary source of truth for Expo-specific work.

```text
Expo official skills
= source of truth for Expo / EAS / React Native implementation details

Our project skills
= orchestration, product workflow, shared app rules, and review process
```

Do not rewrite Expo-specific guidance unless there is a clear project-specific reason.

---

## Official Sources

- Expo Skills documentation: https://docs.expo.dev/skills/
- Expo Skills GitHub repository: https://github.com/expo/skills
- Expo building-native-ui skill:
  https://github.com/expo/skills/blob/main/plugins/expo/skills/building-native-ui/SKILL.md
- Expo native-data-fetching skill:
  https://github.com/expo/skills/blob/main/plugins/expo/skills/native-data-fetching/SKILL.md
- Expo expo-cicd-workflows skill:
  https://github.com/expo/skills/blob/main/plugins/expo/skills/expo-cicd-workflows/SKILL.md
- Expo expo-deployment skill:
  https://github.com/expo/skills/blob/main/plugins/expo/skills/expo-deployment/SKILL.md
- Expo dev-client skill:
  https://github.com/expo/skills/tree/main/plugins/expo/skills/expo-dev-client
- Expo upgrading-expo skill:
  https://github.com/expo/skills/tree/main/plugins/expo/skills/upgrading-expo
- Expo tailwind setup skill:
  https://github.com/expo/skills/tree/main/plugins/expo/skills/expo-tailwind-setup
- Expo EAS update insights skill:
  https://github.com/expo/skills/tree/main/plugins/expo/skills/eas-update-insights

---

## Installation

### Claude Code

Use the Expo plugin marketplace:

```text
/plugin marketplace add expo/skills
/plugin install expo
```

### Cursor

Install Expo Skills from GitHub as a remote rule:

```text
https://github.com/expo/skills.git
```

Then verify the skills appear in Cursor settings under rules/skills/subagents.

### Codex or other agents

Use the skills CLI:

```bash
npx skills add expo/skills
```

or, depending on package manager:

```bash
bunx skills add expo/skills
pnpm dlx skills add expo/skills
yarn dlx skills add expo/skills
```

After installation, ask the agent Expo-specific questions and let it auto-discover the correct
skill.

---

## Official Expo Skills and Responsibilities

| Official Expo Skill       | Main Responsibility                                                                                      | Use When                                                                 |
| ------------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `building-native-ui`      | Build native UI with Expo Router, styling, components, navigation, animations, patterns, and native tabs | Creating screens, navigation, UI components, native-feeling interactions |
| `native-data-fetching`    | API calls, fetch, React Query, SWR, caching, offline support, token/auth handling, Expo Router loaders   | Implementing or debugging network requests or API integration            |
| `expo-cicd-workflows`     | EAS workflow YAML files and CI/CD automation for Expo/EAS                                                | Creating `.eas/workflows/*.yml`, EAS build pipelines, automation         |
| `expo-deployment`         | Deploy Expo apps to iOS App Store, Android Play Store, web hosting, and API routes                       | Building production apps, submitting to stores, release setup            |
| `expo-dev-client`         | Build and distribute Expo development clients locally or via TestFlight                                  | Using custom native modules or development builds                        |
| `expo-tailwind-setup`     | Set up Tailwind CSS v4 / NativeWind in Expo                                                              | Adding Tailwind-style universal styling to Expo                          |
| `upgrading-expo`          | Upgrade Expo SDK and fix dependency issues                                                               | Moving to a newer Expo SDK                                               |
| `eas-update-insights`     | Check health of published EAS Updates                                                                    | Monitoring OTA update health, crash rates, install/launch counts         |
| `expo-api-routes`         | Create API routes in Expo Router with EAS Hosting                                                        | Building API routes inside Expo Router                                   |
| `expo-module`             | Create native Expo modules and native views                                                              | Building Swift/Kotlin/TypeScript native modules                          |
| `use-dom`                 | Run web code in a WebView on native and as-is on web                                                     | Reusing web components inside Expo native apps                           |
| `expo-ui-swift-ui`        | Use SwiftUI views/modifiers through Expo UI                                                              | iOS-native UI integration                                                |
| `expo-ui-jetpack-compose` | Use Jetpack Compose views/modifiers through Expo UI                                                      | Android-native UI integration                                            |

---

## Recommended Workflow

### New Expo mobile project

```text
1. building-native-ui
   → create app structure, screens, navigation, native UI patterns

2. native-data-fetching
   → add API client, auth/token handling, caching, loading/error states

3. expo-dev-client
   → create development build if native modules or custom config are needed

4. expo-cicd-workflows
   → set up EAS workflows for builds and automation

5. expo-deployment
   → prepare app store build, submit flow, release setup
```

### New screen or feature

```text
1. building-native-ui
   → create screen, navigation, UI layout, native interaction pattern

2. native-data-fetching
   → connect the screen to backend/API if needed

3. app-design-system or design skills
   → apply product visual direction if the screen needs custom design

4. app-code-review
   → review UX, accessibility, state handling, and code quality
```

### API or data fetching feature

```text
1. app-api-contract-workflow
   → define or review backend ↔ app API contract

2. native-data-fetching
   → implement API calls, cache, React Query/SWR, token handling, offline behavior

3. app-code-review
   → review error handling, loading states, auth/session safety, and user-facing behavior
```

### CI/CD and build automation

```text
1. expo-cicd-workflows
   → create or update .eas/workflows/*.yml

2. expo-deployment
   → connect workflows to build/submit/deploy strategy

3. app-code-review or release review
   → verify secrets, environment variables, release safety, and build profiles
```

### Store release

```text
1. expo-deployment
   → production builds, submit to App Store / Play Store, store metadata

2. expo-dev-client
   → development clients or TestFlight/internal builds if needed

3. eas-update-insights
   → monitor OTA update health after rollout
```

### Upgrade Expo SDK

```text
1. upgrading-expo
   → upgrade SDK and dependencies

2. building-native-ui
   → fix UI/navigation changes if needed

3. native-data-fetching
   → fix network/data-fetching behavior if affected

4. expo-cicd-workflows
   → update build workflows if needed
```

---

## How This Works With Our Skill System

Use Expo official skills for Expo-specific implementation details.

Use our shared project skills for cross-platform workflow and quality gates.

| Area                           | Preferred Skill                      |
| ------------------------------ | ------------------------------------ |
| Expo screen/UI/navigation      | `building-native-ui`                 |
| Expo API/network/data fetching | `native-data-fetching`               |
| Expo build workflows           | `expo-cicd-workflows`                |
| Expo app store deployment      | `expo-deployment`                    |
| Expo development builds        | `expo-dev-client`                    |
| Product API contract           | `app-api-contract-workflow`          |
| Product design direction       | `app-design-system` or design skills |
| Final app code review          | `app-code-review`                    |
| Backend implementation         | backend skills                       |

Do not create a custom Expo architecture skill unless the official skills are missing something
important for the project.

---

## Common Prompts

### Build a screen

```text
Use the official Expo building-native-ui skill.

Build a settings screen with:
- Expo Router navigation
- form inputs
- save button
- loading state
- error state
- success feedback
- native-feeling layout

Use existing design tokens/components if available.
Do not invent backend API contract.
```

### Add API data fetching

```text
Use the official Expo native-data-fetching skill.

Implement data fetching for this screen:
- endpoint: <endpoint>
- auth: <required | not required>
- cache behavior: <fresh | cached | offline-capable>
- loading state: required
- error state: required
- empty state: required

If the API contract is unclear, use app-api-contract-workflow first.
```

### Set up Expo CI/CD

```text
Use the official Expo expo-cicd-workflows skill.

Create EAS workflow files for:
- PR preview build
- development build
- production build
- manual production submit

Use .eas/workflows/*.yml.
Validate against the current EAS workflow schema.
Do not guess unsupported workflow fields.
```

### Deploy to stores

```text
Use the official Expo expo-deployment skill.

Prepare this Expo app for:
- iOS App Store / TestFlight
- Android Play Store internal track
- production build profile
- submit configuration
- app metadata checklist

Do not hard-code secrets.
List required credentials and environment variables.
```

### Create development client

```text
Use the official Expo expo-dev-client skill.

Set up a development client for this Expo app because we need:
- custom native modules
- native config plugins
- TestFlight/internal distribution

Explain required commands, build profile changes, and how developers install the dev client.
```

### Upgrade Expo SDK

```text
Use the official Expo upgrading-expo skill.

Upgrade this project to the latest compatible Expo SDK.
Check:
- package versions
- config changes
- native module compatibility
- EAS build impact
- breaking changes

Do not change unrelated app code.
```

### Check OTA update health

```text
Use the official Expo eas-update-insights skill.

Check the health of the latest EAS Update for:
- crash rate
- install count
- launch count
- unique users
- embedded vs OTA users
- rollout risk

Summarize whether the rollout looks healthy.
```

---

## Project Types

### Mobile only

Use:

```text
building-native-ui
native-data-fetching
expo-dev-client
expo-cicd-workflows
expo-deployment
eas-update-insights
```

### Backend + Mobile

Use:

```text
skills/backend/*
app-api-contract-workflow
building-native-ui
native-data-fetching
expo-cicd-workflows
expo-deployment
app-code-review
```

### Backend + Web + Mobile

Use:

```text
skills/backend/*
skills/web/*
app-api-contract-workflow
app-design-system
building-native-ui
native-data-fetching
expo-cicd-workflows
expo-deployment
app-code-review
```

Do not invoke all skills at once. Use the skill that matches the current phase.

---

## Rules

### Do

- Use official Expo Skills for Expo-specific implementation.
- Use `building-native-ui` for screens, navigation, UI components, animations, and native UI
  patterns.
- Use `native-data-fetching` for API calls, caching, offline behavior, auth tokens, and network
  debugging.
- Use `expo-cicd-workflows` for `.eas/workflows/*.yml`.
- Use `expo-deployment` for app store, TestFlight, Play Store, web deployment, and production build
  guidance.
- Use `expo-dev-client` when native modules or custom native configuration are involved.
- Use `upgrading-expo` for SDK upgrades.
- Use `eas-update-insights` after OTA update rollout.
- Use our shared skills for product-level API contract, design system, and final code review.

### Do Not

- Do not recreate official Expo skills unless necessary.
- Do not guess EAS workflow schema fields.
- Do not hard-code secrets or credentials into examples.
- Do not use app store deployment guidance for normal feature development.
- Do not use CI/CD skills when only creating a screen.
- Do not use `building-native-ui` to define backend API contracts.
- Do not use `native-data-fetching` to redesign UI layout.
- Do not submit to stores without explicit user approval.

---

## Suggested AGENTS.md Section

Copy this into a project `AGENTS.md` when the project uses Expo.

```md
## Expo / React Native Skills

This project uses official Expo Skills as the source of truth for Expo-specific work.

Use these skills when relevant:

- `building-native-ui`: Use for Expo Router screens, native UI components, styling, navigation,
  animations, patterns, and native tabs.

- `native-data-fetching`: Use for API requests, fetch, React Query, SWR, error handling, caching,
  offline behavior, token/auth handling, and Expo Router loaders.

- `expo-cicd-workflows`: Use for `.eas/workflows/*.yml`, EAS build pipelines, workflow automation,
  triggers, jobs, concurrency, and validation.

- `expo-deployment`: Use for iOS App Store, TestFlight, Android Play Store, web deployment, EAS
  Build, EAS Submit, and app metadata.

- `expo-dev-client`: Use for Expo development builds and distribution when native modules or custom
  native config are required.

- `upgrading-expo`: Use for Expo SDK upgrades and dependency compatibility.

- `eas-update-insights`: Use to evaluate OTA update health after publishing EAS Updates.

Project rules:

- Do not recreate official Expo guidance unless a project-specific constraint requires it.
- Use `app-api-contract-workflow` before implementing mobile API integration if the API contract is
  unclear.
- Use design skills before UI implementation if the visual direction is unclear.
- Use `app-code-review` before finishing mobile app changes.
- Do not fabricate EAS build, submit, or update results.
```

---

## Minimal Prompt Template

Use this when starting a mobile feature:

```text
I want to build an Expo mobile feature.

Feature:
- <feature>

Product context:
- <context>

Target users:
- <users>

Use official Expo Skills directly:
1. Use building-native-ui for screen, navigation, and native UI structure.
2. Use native-data-fetching if the feature calls an API.
3. Use app-api-contract-workflow first if the backend contract is unclear.
4. Use design skills if visual direction is unclear.
5. Use app-code-review before finishing.

Do not mix responsibilities:
- Expo official skills handle Expo-specific implementation.
- Shared app skills handle product contract, design direction, and final review.
- Backend skills handle backend implementation.
```

---

## Quick Decision Guide

| User Request                                    | Use This Official Expo Skill |
| ----------------------------------------------- | ---------------------------- |
| “Build a settings screen with navigation”       | `building-native-ui`         |
| “Add API calls and caching”                     | `native-data-fetching`       |
| “Create EAS workflow for PR builds”             | `expo-cicd-workflows`        |
| “Deploy to TestFlight / App Store / Play Store” | `expo-deployment`            |
| “Create a development build”                    | `expo-dev-client`            |
| “Upgrade Expo SDK”                              | `upgrading-expo`             |
| “Check if the latest OTA update is healthy”     | `eas-update-insights`        |
| “Use Tailwind/NativeWind in Expo”               | `expo-tailwind-setup`        |
| “Create native module/view”                     | `expo-module`                |
| “Reuse web code in native app”                  | `use-dom`                    |

---

## Summary

Use official Expo Skills directly for Expo-specific work.

```text
building-native-ui
= Expo screen, UI, navigation, native interaction patterns

native-data-fetching
= API calls, cache, offline, auth/token handling

expo-cicd-workflows
= EAS workflow YAML and automation

expo-deployment
= app store, TestFlight, Play Store, EAS build/submit/deploy

expo-dev-client
= development builds and native-module development workflow
```

Our custom skills should only orchestrate product workflow, API contracts, design direction, and
final review around the official Expo skills.
