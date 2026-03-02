#!/usr/bin/env bash
set -euo pipefail

ZSHRC="$HOME/.zshrc"

if [[ $# -lt 2 ]]; then
  echo "Usage: $0 <alias_name> <execution>"
  exit 1
fi

ALIAS_NAME="$1"
EXECUTION="$2"

if [[ ! "$ALIAS_NAME" =~ ^[a-zA-Z_][a-zA-Z0-9_]*$ ]]; then
  echo "Invalid alias name: '$ALIAS_NAME'"
  echo "Allowed pattern: [a-zA-Z_][a-zA-Z0-9_]*"
  exit 1
fi

escape_single_quotes() {
  local s="$1"
  s="${s//\'/\'\\\'\'}"
  printf '%s' "$s"
}

ESCAPED_EXECUTION="$(escape_single_quotes "$EXECUTION")"
USER_ALIAS="alias ${ALIAS_NAME}='${ESCAPED_EXECUTION}'"

ensure_alias() {
  local alias_name="$1"
  local alias_line="$2"

  if [[ -f "$ZSHRC" ]] && grep -Eq "^[[:space:]]*alias[[:space:]]+${alias_name}=" "$ZSHRC"; then
    # Replace existing alias definition.
    sed -i '' -E "s|^[[:space:]]*alias[[:space:]]+${alias_name}=.*$|${alias_line}|" "$ZSHRC"
  else
    {
      echo ""
      echo "$alias_line"
    } >> "$ZSHRC"
  fi
}

ensure_alias "$ALIAS_NAME" "$USER_ALIAS"

IS_SOURCED=0
if [[ -n "${ZSH_EVAL_CONTEXT:-}" && "${ZSH_EVAL_CONTEXT}" == *":file" ]]; then
  IS_SOURCED=1
elif [[ -n "${BASH_VERSION:-}" ]]; then
  if (return 0 2>/dev/null); then
    IS_SOURCED=1
  fi
fi

if [[ "$IS_SOURCED" -eq 1 ]]; then
  source "$ZSHRC"
  clear
  echo "Done. Aliases are configured and loaded: ${ALIAS_NAME}"
else
  clear
  echo "Done. Aliases are configured: ${ALIAS_NAME}"
  echo "To refresh current terminal run: source ~/.zshrc"
fi

# Ask for confirmation before closing only in interactive mode.
if [[ -t 0 && -t 1 ]]; then
  read -r -p "Press Enter to close the script..."
fi
