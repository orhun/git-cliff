## {{ release_title | default(value="[unreleased]") }}
{% for type, commits in changes | group_by(attribute="commit_type") %}
### {{ type | capitalize}}
{% for commit in commits %}
- {{ commit.message | capitalize }}{% endfor %}
{% endfor %}