## {{ version | default(value="[unreleased]") }}
{% for type, commits in commits | group_by(attribute="commit_type") %}
### {{ type | capitalize}}
{% for commit in commits %}
- {{ commit.message | capitalize }}{% endfor %}
{% endfor %}