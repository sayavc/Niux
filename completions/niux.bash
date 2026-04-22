_niux_completions() {
  local cur="${COMP_WORDS[COMP_CWORD]}"
  local prev="${COMP_WORDS[COMP_CWORD-1]}"
  
  case "$prev" in
    -l|-Hl|-Sl|-Hr|-Sr|-Hra|-Sra)
      COMPREPLY=($(compgen -W "$(niux -l)" -- "$cur"))
      ;;
    -Hi|-Si|-Hia|-Sia)
      COMPREPLY=($(compgen -W "$(niux --search $cur)" -- "$cur"))
      ;;
  esac
}

complete -F _niux_completions niux
