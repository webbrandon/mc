pub mod process;

pub use process::CompletionProcess;

pub struct Completions {
    
}

impl Completions {
    pub fn bash() {
        println!("{}",r#"
_mc() {
    local i cur prev opts cmds
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${i}" in
            mc)
                cmd="mc"
                ;;
            
            bash)
                cmd+="__bash"
                ;;
            completions)
                cmd+="__completions"
                ;;
            create)
                cmd+="__create"
                ;;
            elvish)
                cmd+="__elvish"
                ;;
            fish)
                cmd+="__fish"
                ;;
            powershell)
                cmd+="__powershell"
                ;;
            zsh)
                cmd+="__zsh"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        mc)
            opts=" -n -m -h -c -e -f -r -i -d -s -t -o -p -l  --no-prompt --mute --help --config --env --flow --repo --image --docker --script --template --template-out --param --post-script   create completions"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --env)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -e)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --flow)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --repo)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --image)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -i)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --docker)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --script)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --template)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --template-out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --param)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -p)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --post-script)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                    -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        
        mc__completions)
            opts=" -h -V  --help --version   bash fish zsh powershell elvish"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mc__completions__bash)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mc__completions__elvish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mc__completions__fish)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mc__completions__powershell)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mc__completions__zsh)
            opts=" -h -V  --help --version  <name> "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        mc__create)
            opts=" -g -h -V  --guide --help --version --api  "
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                
                --api)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

complete -F _mc -o bashdefault -o default mc

"#);
}

pub fn fish() {
    println!("{}",r#"
complete -c mc -n "__fish_use_subcommand" -s c -l config -d 'Define which api config file to use.'
complete -c mc -n "__fish_use_subcommand" -s e -l env -d 'Load dot env file. Overrides env-prompts api configs.'
complete -c mc -n "__fish_use_subcommand" -s f -l flow -d 'Use flow pattern.'
complete -c mc -n "__fish_use_subcommand" -s r -l repo -d 'Clone git repository.'
complete -c mc -n "__fish_use_subcommand" -s i -l image -d 'Container image to use when running flow.'
complete -c mc -n "__fish_use_subcommand" -s d -l docker -d 'Docker image to run in.'
complete -c mc -n "__fish_use_subcommand" -s s -l script -d 'Sets the script to run the the start.'
complete -c mc -n "__fish_use_subcommand" -s t -l template -d 'Sets a custom template file.'
complete -c mc -n "__fish_use_subcommand" -s o -l template-out -d 'Rendered template out file write location.'
complete -c mc -n "__fish_use_subcommand" -s p -l param -d 'Sets a custom template parameters file.'
complete -c mc -n "__fish_use_subcommand" -s l -l post-script -d 'Sets the script to run at the end.'
complete -c mc -n "__fish_use_subcommand" -s n -l no-prompt -d 'Proceed prompts default to yes and env-prompts are disabled.'
complete -c mc -n "__fish_use_subcommand" -s m -l mute -d 'Silence output.'
complete -c mc -n "__fish_use_subcommand" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_use_subcommand" -f -a "create" -d 'Create api files.'
complete -c mc -n "__fish_use_subcommand" -f -a "completions" -d 'Completion scripts for various shells.'
complete -c mc -n "__fish_seen_subcommand_from create" -l api -d 'What API type you would like to create.'
complete -c mc -n "__fish_seen_subcommand_from create" -s g -l guide -d 'Use the api create guide.'
complete -c mc -n "__fish_seen_subcommand_from create" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from create" -s V -l version -d 'Prints version information'
complete -c mc -n "__fish_seen_subcommand_from completions" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from completions" -s V -l version -d 'Prints version information'
complete -c mc -n "__fish_seen_subcommand_from completions" -f -a "bash" -d 'Bash completion script.'
complete -c mc -n "__fish_seen_subcommand_from completions" -f -a "fish" -d 'Fish completion script.'
complete -c mc -n "__fish_seen_subcommand_from completions" -f -a "zsh" -d 'Zsh completion script.'
complete -c mc -n "__fish_seen_subcommand_from completions" -f -a "powershell" -d 'PowerShell completion script.'
complete -c mc -n "__fish_seen_subcommand_from completions" -f -a "elvish" -d 'Elvish completion script.'
complete -c mc -n "__fish_seen_subcommand_from bash" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from bash" -s V -l version -d 'Prints version information'
complete -c mc -n "__fish_seen_subcommand_from fish" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from fish" -s V -l version -d 'Prints version information'
complete -c mc -n "__fish_seen_subcommand_from zsh" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from zsh" -s V -l version -d 'Prints version information'
complete -c mc -n "__fish_seen_subcommand_from powershell" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from powershell" -s V -l version -d 'Prints version information'
complete -c mc -n "__fish_seen_subcommand_from elvish" -s h -l help -d 'Prints help information'
complete -c mc -n "__fish_seen_subcommand_from elvish" -s V -l version -d 'Prints version information'

"#);
}

pub fn zsh() {
    println!("{}",r#"
#compdef mc

autoload -U is-at-least

_mc() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-c+[Define which api config file to use.]' \
'--config=[Define which api config file to use.]' \
'-e+[Load dot env file. Overrides env-prompts api configs.]' \
'--env=[Load dot env file. Overrides env-prompts api configs.]' \
'-f+[Use flow pattern.]' \
'--flow=[Use flow pattern.]' \
'-r+[Clone git repository.]' \
'--repo=[Clone git repository.]' \
'-i+[Container image to use when running flow.]' \
'--image=[Container image to use when running flow.]' \
'-d+[Docker image to run in.]' \
'--docker=[Docker image to run in.]' \
'-s+[Sets the script to run the the start.]' \
'--script=[Sets the script to run the the start.]' \
'-t+[Sets a custom template file.]' \
'--template=[Sets a custom template file.]' \
'-o+[Rendered template out file write location.]' \
'--template-out=[Rendered template out file write location.]' \
'-p+[Sets a custom template parameters file.]' \
'--param=[Sets a custom template parameters file.]' \
'-l+[Sets the script to run at the end.]' \
'--post-script=[Sets the script to run at the end.]' \
'-n[Proceed prompts default to yes and env-prompts are disabled.]' \
'--no-prompt[Proceed prompts default to yes and env-prompts are disabled.]' \
'-m[Silence output.]' \
'--mute[Silence output.]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
":: :_mc_commands" \
"*::: :->mc" \
&& ret=0
    case $state in
    (mc)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:mc-command-$line[1]:"
        case $line[1] in
            (create)
_arguments "${_arguments_options[@]}" \
'--api=[What API type you would like to create.]' \
'-g[Use the api create guide.]' \
'--guide[Use the api create guide.]' \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
&& ret=0
;;
(completions)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
":: :_mc__completions_commands" \
"*::: :->completions" \
&& ret=0
case $state in
    (completions)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:mc-completions-command-$line[1]:"
        case $line[1] in
            (bash)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Bash completion script.:_files' \
&& ret=0
;;
(fish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Fish completion script.:_files' \
&& ret=0
;;
(zsh)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Zsh completion script.:_files' \
&& ret=0
;;
(powershell)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- PowerShell completion script.:_files' \
&& ret=0
;;
(elvish)
_arguments "${_arguments_options[@]}" \
'-h[Prints help information]' \
'--help[Prints help information]' \
'-V[Prints version information]' \
'--version[Prints version information]' \
'::name -- Elvish completion script.:_files' \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_mc_commands] )) ||
_mc_commands() {
    local commands; commands=(
        "create:Create api files." \
"completions:Completion scripts for various shells." \
    )
    _describe -t commands 'mc commands' commands "$@"
}
(( $+functions[_mc__completions__bash_commands] )) ||
_mc__completions__bash_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'mc completions bash commands' commands "$@"
}
(( $+functions[_mc__completions_commands] )) ||
_mc__completions_commands() {
    local commands; commands=(
        "bash:Bash completion script." \
"fish:Fish completion script." \
"zsh:Zsh completion script." \
"powershell:PowerShell completion script." \
"elvish:Elvish completion script." \
    )
    _describe -t commands 'mc completions commands' commands "$@"
}
(( $+functions[_mc__create_commands] )) ||
_mc__create_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'mc create commands' commands "$@"
}
(( $+functions[_mc__completions__elvish_commands] )) ||
_mc__completions__elvish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'mc completions elvish commands' commands "$@"
}
(( $+functions[_mc__completions__fish_commands] )) ||
_mc__completions__fish_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'mc completions fish commands' commands "$@"
}
(( $+functions[_mc__completions__powershell_commands] )) ||
_mc__completions__powershell_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'mc completions powershell commands' commands "$@"
}
(( $+functions[_mc__completions__zsh_commands] )) ||
_mc__completions__zsh_commands() {
    local commands; commands=(
        
    )
    _describe -t commands 'mc completions zsh commands' commands "$@"
}

_mc "$@"
"#);
}

pub fn powershell() {
    println!("{}",r#"

using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'mc' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'mc'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-')) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'mc' {
            [CompletionResult]::new('-c', 'c', [CompletionResultType]::ParameterName, 'Define which api config file to use.')
            [CompletionResult]::new('--config', 'config', [CompletionResultType]::ParameterName, 'Define which api config file to use.')
            [CompletionResult]::new('-e', 'e', [CompletionResultType]::ParameterName, 'Load dot env file. Overrides env-prompts api configs.')
            [CompletionResult]::new('--env', 'env', [CompletionResultType]::ParameterName, 'Load dot env file. Overrides env-prompts api configs.')
            [CompletionResult]::new('-f', 'f', [CompletionResultType]::ParameterName, 'Use flow pattern.')
            [CompletionResult]::new('--flow', 'flow', [CompletionResultType]::ParameterName, 'Use flow pattern.')
            [CompletionResult]::new('-r', 'r', [CompletionResultType]::ParameterName, 'Clone git repository.')
            [CompletionResult]::new('--repo', 'repo', [CompletionResultType]::ParameterName, 'Clone git repository.')
            [CompletionResult]::new('-i', 'i', [CompletionResultType]::ParameterName, 'Container image to use when running flow.')
            [CompletionResult]::new('--image', 'image', [CompletionResultType]::ParameterName, 'Container image to use when running flow.')
            [CompletionResult]::new('-d', 'd', [CompletionResultType]::ParameterName, 'Docker image to run in.')
            [CompletionResult]::new('--docker', 'docker', [CompletionResultType]::ParameterName, 'Docker image to run in.')
            [CompletionResult]::new('-s', 's', [CompletionResultType]::ParameterName, 'Sets the script to run the the start.')
            [CompletionResult]::new('--script', 'script', [CompletionResultType]::ParameterName, 'Sets the script to run the the start.')
            [CompletionResult]::new('-t', 't', [CompletionResultType]::ParameterName, 'Sets a custom template file.')
            [CompletionResult]::new('--template', 'template', [CompletionResultType]::ParameterName, 'Sets a custom template file.')
            [CompletionResult]::new('-o', 'o', [CompletionResultType]::ParameterName, 'Rendered template out file write location.')
            [CompletionResult]::new('--template-out', 'template-out', [CompletionResultType]::ParameterName, 'Rendered template out file write location.')
            [CompletionResult]::new('-p', 'p', [CompletionResultType]::ParameterName, 'Sets a custom template parameters file.')
            [CompletionResult]::new('--param', 'param', [CompletionResultType]::ParameterName, 'Sets a custom template parameters file.')
            [CompletionResult]::new('-l', 'l', [CompletionResultType]::ParameterName, 'Sets the script to run at the end.')
            [CompletionResult]::new('--post-script', 'post-script', [CompletionResultType]::ParameterName, 'Sets the script to run at the end.')
            [CompletionResult]::new('-n', 'n', [CompletionResultType]::ParameterName, 'Proceed prompts default to yes and env-prompts are disabled.')
            [CompletionResult]::new('--no-prompt', 'no-prompt', [CompletionResultType]::ParameterName, 'Proceed prompts default to yes and env-prompts are disabled.')
            [CompletionResult]::new('-m', 'm', [CompletionResultType]::ParameterName, 'Silence output.')
            [CompletionResult]::new('--mute', 'mute', [CompletionResultType]::ParameterName, 'Silence output.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('create', 'create', [CompletionResultType]::ParameterValue, 'Create api files.')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Completion scripts for various shells.')
            break
        }
        'mc;create' {
            [CompletionResult]::new('--api', 'api', [CompletionResultType]::ParameterName, 'What API type you would like to create.')
            [CompletionResult]::new('-g', 'g', [CompletionResultType]::ParameterName, 'Use the api create guide.')
            [CompletionResult]::new('--guide', 'guide', [CompletionResultType]::ParameterName, 'Use the api create guide.')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mc;completions' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('bash', 'bash', [CompletionResultType]::ParameterValue, 'Bash completion script.')
            [CompletionResult]::new('fish', 'fish', [CompletionResultType]::ParameterValue, 'Fish completion script.')
            [CompletionResult]::new('zsh', 'zsh', [CompletionResultType]::ParameterValue, 'Zsh completion script.')
            [CompletionResult]::new('powershell', 'powershell', [CompletionResultType]::ParameterValue, 'PowerShell completion script.')
            [CompletionResult]::new('elvish', 'elvish', [CompletionResultType]::ParameterValue, 'Elvish completion script.')
            break
        }
        'mc;completions;bash' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mc;completions;fish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mc;completions;zsh' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mc;completions;powershell' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
        'mc;completions;elvish' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Prints help information')
            [CompletionResult]::new('-V', 'V', [CompletionResultType]::ParameterName, 'Prints version information')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'Prints version information')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}

"#);
}

pub fn elvish() {
    println!("{}",r#"

edit:completion:arg-completer[mc] = [@words]{
    fn spaces [n]{
        repeat $n ' ' | joins ''
    }
    fn cand [text desc]{
        edit:complex-candidate $text &display-suffix=' '(spaces (- 14 (wcswidth $text)))$desc
    }
    command = 'mc'
    for word $words[1:-1] {
        if (has-prefix $word '-') {
            break
        }
        command = $command';'$word
    }
    completions = [
        &'mc'= {
            cand -c 'Define which api config file to use.'
            cand --config 'Define which api config file to use.'
            cand -e 'Load dot env file. Overrides env-prompts api configs.'
            cand --env 'Load dot env file. Overrides env-prompts api configs.'
            cand -f 'Use flow pattern.'
            cand --flow 'Use flow pattern.'
            cand -r 'Clone git repository.'
            cand --repo 'Clone git repository.'
            cand -i 'Container image to use when running flow.'
            cand --image 'Container image to use when running flow.'
            cand -d 'Docker image to run in.'
            cand --docker 'Docker image to run in.'
            cand -s 'Sets the script to run the the start.'
            cand --script 'Sets the script to run the the start.'
            cand -t 'Sets a custom template file.'
            cand --template 'Sets a custom template file.'
            cand -o 'Rendered template out file write location.'
            cand --template-out 'Rendered template out file write location.'
            cand -p 'Sets a custom template parameters file.'
            cand --param 'Sets a custom template parameters file.'
            cand -l 'Sets the script to run at the end.'
            cand --post-script 'Sets the script to run at the end.'
            cand -n 'Proceed prompts default to yes and env-prompts are disabled.'
            cand --no-prompt 'Proceed prompts default to yes and env-prompts are disabled.'
            cand -m 'Silence output.'
            cand --mute 'Silence output.'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand create 'Create api files.'
            cand completions 'Completion scripts for various shells.'
        }
        &'mc;create'= {
            cand --api 'What API type you would like to create.'
            cand -g 'Use the api create guide.'
            cand --guide 'Use the api create guide.'
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mc;completions'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
            cand bash 'Bash completion script.'
            cand fish 'Fish completion script.'
            cand zsh 'Zsh completion script.'
            cand powershell 'PowerShell completion script.'
            cand elvish 'Elvish completion script.'
        }
        &'mc;completions;bash'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mc;completions;fish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mc;completions;zsh'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mc;completions;powershell'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
        &'mc;completions;elvish'= {
            cand -h 'Prints help information'
            cand --help 'Prints help information'
            cand -V 'Prints version information'
            cand --version 'Prints version information'
        }
    ]
    $completions[$command]
}

"#);
}
}
