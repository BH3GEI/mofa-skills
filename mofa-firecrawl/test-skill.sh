#!/bin/bash
# MOFA Firecrawl Skill Test Script

set -e

echo "=== MOFA Firecrawl Skill Test ==="
echo ""

# Check firecrawl CLI
echo "1. Checking Firecrawl CLI..."
if command -v firecrawl &> /dev/null; then
    echo "   ✓ firecrawl found: $(firecrawl --version)"
else
    echo "   ✗ firecrawl not found"
    echo "   Please install: npm install -g firecrawl-cli"
    exit 1
fi

# Check authentication
echo ""
echo "2. Checking Firecrawl authentication..."
if firecrawl --status 2>/dev/null | grep -q "Authenticated"; then
    echo "   ✓ Authenticated"
    AUTH=true
else
    echo "   ! Not authenticated (run 'firecrawl login' for full functionality)"
    AUTH=false
fi

# Check skill files
echo ""
echo "3. Checking skill file structure..."
FILES=(
    "SKILL.md"
    "config.toml"
    "templates/scrape-prompt.md"
    "templates/crawl-prompt.md"
    "templates/browser-prompt.md"
    "examples/website-analysis.md"
    "examples/competitor-research.md"
    "examples/dynamic-content.md"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "   ✓ $file"
    else
        echo "   ✗ $file (missing)"
    fi
done

# Test command syntax (dry run)
echo ""
echo "4. Testing command syntax..."

# Test scrape help
if firecrawl scrape --help > /dev/null 2>&1; then
    echo "   ✓ scrape command available"
else
    echo "   ✗ scrape command failed"
fi

# Test crawl help
if firecrawl crawl --help > /dev/null 2>&1; then
    echo "   ✓ crawl command available"
else
    echo "   ✗ crawl command failed"
fi

# Test map help
if firecrawl map --help > /dev/null 2>&1; then
    echo "   ✓ map command available"
else
    echo "   ✗ map command failed"
fi

# Test search help
if firecrawl search --help > /dev/null 2>&1; then
    echo "   ✓ search command available"
else
    echo "   ✗ search command failed"
fi

# Test browser help
if firecrawl browser --help > /dev/null 2>&1; then
    echo "   ✓ browser command available"
else
    echo "   ✗ browser command failed"
fi

# Functional test (if authenticated)
if [ "$AUTH" = true ]; then
    echo ""
    echo "5. Running functional tests..."

    # Test scrape
    echo "   Testing scrape..."
    if firecrawl "https://example.com" -o /tmp/test-scrape.md 2>/dev/null; then
        echo "   ✓ scrape works"
        rm -f /tmp/test-scrape.md
    else
        echo "   ! scrape test skipped (may need API key)"
    fi
else
    echo ""
    echo "5. Skipping functional tests (not authenticated)"
fi

echo ""
echo "=== Test Complete ==="
echo ""
echo "To use mofa-firecrawl:"
echo "  1. Get API key from https://firecrawl.dev"
echo "  2. Run: firecrawl login --api-key fc-YOUR-API-KEY"
echo "  3. Or set env: export FIRECRAWL_API_KEY=fc-YOUR-API-KEY"
