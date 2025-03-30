<script>
    export const createEthernetEdge = (params, edgeId) => {
        return {
            id: edgeId,
            ...params,
            type: 'straight',
            animated: false,
            style: {
                stroke: 'var(--secondary-color)',
                strokeWidth: 2,
            },
        };
    };
</script>
