# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Tauri 2.0 + Next.js 15 desktop application template that combines:
- **Frontend**: Next.js 15 with App Router, React 19, TypeScript, TailwindCSS 4, and shadcn/ui components
- **Backend**: Tauri 2.0 (Rust) for native desktop functionality
- **Package Manager**: pnpm

## Development Commands

### Frontend Development
```bash
# Install dependencies
pnpm install

# Run Next.js dev server (localhost:3000)
pnpm dev

# Run Tauri desktop app in development mode
pnpm tauri dev

# Build for production
pnpm tauri build

# Run tests
pnpm test

# Linting and formatting
pnpm lint        # Run Biome + ESLint checks
pnpm fix         # Fix linting/formatting issues
```

### Adding shadcn/ui Components
```bash
pnpm dlx shadcn@latest add [component-name]
```
Components are installed to `src/components/ui/`

## Architecture

### Frontend Structure
- `src/app/` - Next.js App Router pages and layouts
- `src/components/` - React components including shadcn/ui components
- `src/lib/` - Utility functions and helpers
- `src/styles/` - Global CSS and TailwindCSS configuration
- Static exports to `dist/` directory (SSG only, no SSR)

### Backend Structure
- `src-tauri/` - Rust Tauri application code
- `src-tauri/src/` - Main Rust source files
- `src-tauri/tauri.conf.json` - Tauri configuration

### Key Configuration Files
- `next.config.ts` - Next.js configured for static export with `output: "export"`
- `tsconfig.json` - TypeScript with path alias `@/*` for `src/*`
- `biome.json` - Code formatting and linting for TypeScript
- `components.json` - shadcn/ui configuration

## Important Notes

1. **Static Site Generation Only**: This app uses Next.js static export (SSG). Server-side features like API routes and dynamic routing with SSR are not supported.

2. **Tauri Integration**: When using Tauri APIs from frontend code, ensure they're imported within client-side components to avoid `window is not defined` errors during build.

3. **Image Optimization**: The `next/image` component has `unoptimized: true` set in config since Vercel's image optimization isn't available for static exports.

4. **App Identifier**: Before building for release, change the identifier in `src-tauri/tauri.conf.json` from the default `com.tauri.dev` to your own unique identifier.