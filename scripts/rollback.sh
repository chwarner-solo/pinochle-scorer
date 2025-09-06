#!/bin/bash
# scripts/rollback.sh - Manual rollback script for CloudRUN

set -e

# Configuration
PROJECT_ID="${PROJECT_ID:-your-project-id}"
REGION="${REGION:-us-central1}"
SERVICE_NAME="${SERVICE_NAME:-pinochle-scorer-api}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔄 CloudRUN Rollback Utility${NC}"
echo "================================"
echo "Project: $PROJECT_ID"
echo "Region: $REGION"
echo "Service: $SERVICE_NAME"
echo ""

# Get current traffic allocation
echo -e "${YELLOW}📊 Current traffic allocation:${NC}"
gcloud run services describe $SERVICE_NAME \
  --region=$REGION \
  --format="table(status.traffic.revisionName,status.traffic.percent,status.traffic.tag)"

echo ""

# List recent revisions
echo -e "${YELLOW}📋 Recent revisions:${NC}"
gcloud run revisions list \
  --service=$SERVICE_NAME \
  --region=$REGION \
  --filter="status.conditions.type=Ready AND status.conditions.status=True" \
  --sort-by=~metadata.creationTimestamp \
  --format="table(metadata.name,metadata.creationTimestamp,status.imageDigest:label=IMAGE)" \
  --limit=5

echo ""

# Get available revisions for rollback
REVISIONS=($(gcloud run revisions list \
  --service=$SERVICE_NAME \
  --region=$REGION \
  --filter="status.conditions.type=Ready AND status.conditions.status=True" \
  --sort-by=~metadata.creationTimestamp \
  --format="value(metadata.name)" \
  --limit=5))

if [ ${#REVISIONS[@]} -eq 0 ]; then
  echo -e "${RED}❌ No revisions found${NC}"
  exit 1
fi

# Show rollback options
echo -e "${YELLOW}🎯 Rollback options:${NC}"
echo "1) Quick rollback to previous revision"
echo "2) Choose specific revision"
echo "3) Split traffic for testing"
echo "4) Show revision details"
echo "5) Cancel"
echo ""

read -p "Select option (1-5): " choice

case $choice in
  1)
    if [ ${#REVISIONS[@]} -lt 2 ]; then
      echo -e "${RED}❌ No previous revision available${NC}"
      exit 1
    fi

    PREVIOUS_REVISION=${REVISIONS[1]}
    echo -e "${BLUE}⏪ Rolling back to: $PREVIOUS_REVISION${NC}"

    gcloud run services update-traffic $SERVICE_NAME \
      --region=$REGION \
      --to-revisions=$PREVIOUS_REVISION=100

    echo -e "${GREEN}✅ Rollback completed${NC}"
    ;;

  2)
    echo -e "${YELLOW}Select revision to rollback to:${NC}"
    select revision in "${REVISIONS[@]}"; do
      if [ -n "$revision" ]; then
        echo -e "${BLUE}⏪ Rolling back to: $revision${NC}"

        gcloud run services update-traffic $SERVICE_NAME \
          --region=$REGION \
          --to-revisions=$revision=100

        echo -e "${GREEN}✅ Rollback completed${NC}"
        break
      else
        echo -e "${RED}Invalid selection${NC}"
      fi
    done
    ;;

  3)
    echo -e "${YELLOW}Select revision for traffic split:${NC}"
    select revision in "${REVISIONS[@]}"; do
      if [ -n "$revision" ]; then
        read -p "Traffic percentage for $revision (1-100): " percentage

        if [[ $percentage -ge 1 && $percentage -le 100 ]]; then
          CURRENT_REVISION=${REVISIONS[0]}
          REMAINING=$((100 - percentage))

          echo -e "${BLUE}🔀 Splitting traffic: $revision ($percentage%), $CURRENT_REVISION ($REMAINING%)${NC}"

          gcloud run services update-traffic $SERVICE_NAME \
            --region=$REGION \
            --to-revisions=$revision=$percentage,$CURRENT_REVISION=$REMAINING

          echo -e "${GREEN}✅ Traffic split applied${NC}"
        else
          echo -e "${RED}❌ Invalid percentage${NC}"
        fi
        break
      else
        echo -e "${RED}Invalid selection${NC}"
      fi
    done
    ;;

  4)
    echo -e "${YELLOW}Select revision for details:${NC}"
    select revision in "${REVISIONS[@]}"; do
      if [ -n "$revision" ]; then
        echo -e "${BLUE}📋 Revision details: $revision${NC}"

        gcloud run revisions describe $revision \
          --region=$REGION \
          --format="yaml(metadata.creationTimestamp,metadata.labels,spec.template.spec.containers[0].image,status.conditions)"

        break
      else
        echo -e "${RED}Invalid selection${NC}"
      fi
    done
    ;;

  5)
    echo -e "${YELLOW}🚫 Cancelled${NC}"
    exit 0
    ;;

  *)
    echo -e "${RED}❌ Invalid option${NC}"
    exit 1
    ;;
esac

echo ""
echo -e "${YELLOW}📊 Updated traffic allocation:${NC}"
gcloud run services describe $SERVICE_NAME \
  --region=$REGION \
  --format="table(status.traffic.revisionName,status.traffic.percent,status.traffic.tag)"