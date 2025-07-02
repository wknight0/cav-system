<script>
    export const createEthernetEdge = (params, edgeId) => {
        return {
            id: edgeId,
            ...params,
            type: 'straight',
            connection_type: 'ethernet',
            animated: false,
            style: {
                stroke: params.color || 'var(--primary-text-color)',
                strokeWidth: 2,
                opacity: 0.8,
            },
        };
    };
</script>
