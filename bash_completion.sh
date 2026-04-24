# shellcheck disable=all
# TODO: We will need to re-enable these... at some point. 
_st_complete()
{
    local cur prev words cword
    _get_comp_words_by_ref -n : cur prev words cword 2>/dev/null || {
        cur="${COMP_WORDS[COMP_CWORD]}"
        prev="${COMP_WORDS[COMP_CWORD-1]}"
        words=("${COMP_WORDS[@]}")
        cword=$COMP_CWORD
    }

    local subcommands='add
pull
push
init
list
remove'
    local opts="-T -t"
    local topics_dir="$(pwd)/st/topics"

    if [[ $cword -eq 1  ]]; then
        COMPREPLY=( $(compgen -W "${subcommands[@]}" -- "$cur") )
        return 0
    fi

    local topic=""
    for ((i=1; i < cword; i++)); do
        if [[ "${words[i]}" == "-T" && $((i+1)) -lt $cword ]]; then
            topic="${words[i+1]}"
        fi
    done

    if [[ "$prev" == "-T" ]]; then
        if [[ -d "$topics_dir" ]]; then
            COMPREPLY=( $(compgen -W "$(find "$topics_dir" -mindepth 1 -maxdepth 1 -type d -printf '%f\n')" -- "$cur") )
        fi
        return 0
    fi

    if [[ "$prev" == "-t" ]]; then
        if [[ -n "$topic" && -d "$topics_dir/$topic" ]]; then
            COMPREPLY=( $(compgen -W "$(find "$topics_dir/$topic" -mindepth 1 -maxdepth 1 -type d -printf '%f\n')" -- "$cur") )
        fi
        return 0
    fi

    if [[ "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
        return 0
    fi
}

complete -F _st_complete st

